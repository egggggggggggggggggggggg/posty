use chrono::{DateTime, Utc};
use cookie::Cookie;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::{Client, Method, Request, Response, StatusCode, header::HeaderMap};
use std::{str::FromStr, time::Duration};
pub mod collection;
pub mod executor;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestData {
    pub method: String,
    pub url: String,
    pub params: Vec<KVPair>,
    pub headers: Vec<KVPair>,
    pub body: Option<RequestBody>,
    pub auth: Option<Auth>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KVPair {
    pub key: String,
    pub value: String,
    pub enabled: bool,   // toggle in UI
    pub sensitive: bool, // hide value in UI (e.g. tokens)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestBody {
    None,
    Raw { content_type: String, body: String },
    Form(Vec<KVPair>),
    Json(String), // store raw JSON text for editing
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Auth {
    None,
    Basic {
        username: String,
        password: String,
    },
    Bearer {
        token: String,
    },
    ApiKey {
        key: String,
        value: String,
        in_header: bool, // header vs query param
    },
}
#[derive(Debug)]
pub enum IntoRequestError {
    InvalidMethod,
    BuildError(reqwest::Error),
}
impl Default for RequestData {
    ///Default constructor. Some headers are required by default.
    fn default() -> Self {
        Self {
            method: "GET".to_string(),
            url: String::default(),
            params: Vec::new(),
            headers: Vec::new(),
            body: None,
            auth: None,
        }
    }
}

impl RequestData {
    pub fn into_request(self, client: &Client) -> Result<Request, IntoRequestError> {
        let method = Method::from_str(&self.method).map_err(|_| IntoRequestError::InvalidMethod)?;

        let mut req = client.request(method, &self.url);
        let params: Vec<(&str, &str)> = self
            .params
            .iter()
            .filter(|p| p.enabled)
            .map(|p| (p.key.as_str(), p.value.as_str()))
            .collect();
        if !params.is_empty() {
            req = req.query(&params);
        }
        for h in self.headers.iter().filter(|h| h.enabled) {
            req = req.header(&h.key, &h.value);
        }
        if let Some(auth) = self.auth {
            req = match auth {
                Auth::Basic { username, password } => req.basic_auth(username, Some(password)),
                Auth::Bearer { token } => req.bearer_auth(token),
                Auth::ApiKey {
                    key,
                    value,
                    in_header,
                } => {
                    if in_header {
                        req.header(key, value)
                    } else {
                        req.query(&[(key, value)])
                    }
                }
                Auth::None => req,
            };
        }
        if let Some(body) = self.body {
            req = match body {
                RequestBody::None => req,

                RequestBody::Raw { content_type, body } => {
                    req.header("Content-Type", content_type).body(body)
                }

                RequestBody::Form(fields) => {
                    let form: Vec<(&str, &str)> = fields
                        .iter()
                        .filter(|f| f.enabled)
                        .map(|f| (f.key.as_str(), f.value.as_str()))
                        .collect();

                    req.form(&form)
                }

                RequestBody::Json(json) => {
                    req.header("Content-Type", "application/json").body(json)
                }
            };
        }
        req.build().map_err(IntoRequestError::BuildError)
    }
    pub fn format_body(&mut self) -> Result<(), serde_json::Error> {
        if let Some(body) = &self.body {
            match body {
                RequestBody::Json(j) => {
                    self.body = Some(RequestBody::Json(serde_json::to_string_pretty(j)?));
                }
                _ => {}
            }
        }
        Ok(())
    }
}
///Mainly for the auto complete and dropdown so the users don't have to type out the header.
pub const COMMON_HEADERS: &[&str] = &[
    "Accept",
    "Authorization",
    "Content-Type",
    "User-Agent",
    "Cache-Control",
    "Set-Cookie",
    "",
];
#[derive(Debug)]
pub enum IntoResponseError {
    ConvertError(reqwest::Error),
}
#[derive(Clone)]
pub struct ResponseData<'a> {
    pub body: Option<String>,
    pub cookies: Vec<Cookie<'a>>,
    pub headers: HeaderMap,
    pub status: StatusCode,
    pub timestamp: DateTime<Utc>,
    pub response_time: Duration,
}
impl<'a> ResponseData<'_> {
    pub async fn extract_with_body(
        response_time: Duration,
        response: Response,
    ) -> Result<Self, IntoResponseError> {
        let status = response.status();
        let headers = response.headers().clone();
        let timestamp = Utc::now();
        let cookies = headers
            .get_all(reqwest::header::SET_COOKIE)
            .iter()
            .filter_map(|val| {
                val.to_str()
                    .ok()
                    .and_then(|s| Cookie::parse(s.to_owned()).ok())
            })
            .collect::<Vec<Cookie<'static>>>();
        let bytes = response
            .bytes()
            .await
            .map_err(IntoResponseError::ConvertError)?;
        let body = String::from_utf8_lossy(&bytes).into_owned();
        Ok(Self {
            body: Some(body),
            cookies,
            headers,
            status,
            timestamp,
            response_time,
        })
    }
    //Doesn't extract body to avoid async.
    pub fn extract(latency: Duration, resp: reqwest::Response) -> Self {
        let headers = resp.headers().clone();
        let cookies = headers
            .get_all(reqwest::header::SET_COOKIE)
            .iter()
            .filter_map(|val| {
                val.to_str()
                    .ok()
                    .and_then(|s| Cookie::parse(s.to_owned()).ok())
            })
            .collect::<Vec<Cookie<'static>>>();

        Self {
            status: resp.status(),
            headers: resp.headers().clone(),
            response_time: latency,
            body: None,
            cookies,
            timestamp: Utc::now(),
        }
    }
}
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl Serialize for ResponseData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ResponseData", 7)?;
        // Convert headers to Vec<(String, String)>
        let headers: Vec<(String, String)> = self
            .headers
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or_default().to_string()))
            .collect();
        // Convert cookies to strings
        let cookies: Vec<String> = self.cookies.iter().map(|c| c.to_string()).collect();
        state.serialize_field("body", &self.body)?;
        state.serialize_field("cookies", &cookies)?;
        state.serialize_field("headers", &headers)?;
        state.serialize_field("status", &self.status.as_u16())?;
        state.serialize_field("timestamp", &self.timestamp.to_rfc3339())?;
        state.serialize_field("response_time", &self.response_time.as_millis())?;

        state.end()
    }
}
impl<'de> Deserialize<'de> for ResponseData<'static> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            body: String,
            cookies: Vec<String>,
            headers: Vec<(String, String)>,
            status: u16,
            timestamp: String,
            response_time: u128,
        }
        let helper = Helper::deserialize(deserializer)?;
        let mut header_map = HeaderMap::new();
        for (k, v) in helper.headers {
            header_map.insert(
                k.parse::<HeaderName>().map_err(serde::de::Error::custom)?,
                v.parse::<HeaderValue>().map_err(serde::de::Error::custom)?,
            );
        }
        let cookies = helper
            .cookies
            .into_iter()
            .filter_map(|c| Cookie::parse(c).ok())
            .map(|c| c.into_owned())
            .collect();
        let timestamp = DateTime::parse_from_rfc3339(&helper.timestamp)
            .map_err(serde::de::Error::custom)?
            .with_timezone(&Utc);
        Ok(ResponseData {
            body: Some(helper.body),
            cookies,
            headers: header_map,
            status: StatusCode::from_u16(helper.status).map_err(serde::de::Error::custom)?,
            timestamp,
            response_time: Duration::from_millis(helper.response_time as u64),
        })
    }
}
