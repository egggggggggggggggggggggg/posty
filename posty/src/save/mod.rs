use reqwest::{Client, Method, RequestBuilder};
use serde::de::value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use thiserror::Error; // optional, nice for custom errors

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ApiRequest {
    pub method: String,
    pub url: String,

    #[serde(default)]
    pub headers: HashMap<String, String>,

    #[serde(default)]
    pub query: HashMap<String, String>,

    #[serde(default)]
    pub json: Option<serde_json::Value>,

    #[serde(default)]
    pub form: Option<HashMap<String, String>>,
}
#[derive(Debug, Error)]
pub enum ApiRequestError {
    #[error("invalid HTTP method: {0}")]
    InvalidMethod(String),

    #[error("invalid URL: {0}")]
    InvalidUrl(String),
}

impl ApiRequest {
    fn try_into(self, client: Client) -> Result<RequestBuilder, ApiRequestError> {
        let method = self
            .method
            .parse::<Method>()
            .map_err(|_| ApiRequestError::InvalidMethod(self.method.clone()))?;
        let req = client.request(method, &self.url);
        // headers
        let req = self
            .headers
            .into_iter()
            .fold(req, |r, (k, v)| r.header(&k, &v));
        // query
        let req = if !self.query.is_empty() {
            req.query(&self.query)
        } else {
            req
        };
        // body
        let req = if let Some(json) = self.json {
            req.json(&json)
        } else if let Some(form) = self.form {
            req.form(&form)
        } else {
            req
        };
        // validate URL (reqwest does this lazily, so optional)
        req.try_clone()
            .ok_or_else(|| ApiRequestError::InvalidUrl(self.url.clone()))
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub enum Node {
    File {
        name: String,
        request_info: ApiRequest,
    },
    Dir {
        name: String,
        children: HashMap<String, Node>,
        expanded: bool,
    },
}
impl Node {
    pub fn file(name: impl Into<String>, request_info: ApiRequest) -> Self {
        Node::File {
            name: name.into(),
            request_info,
        }
    }
    pub fn name(&self) -> &str {
        match self {
            Node::File { name, .. } => name,
            Node::Dir { name, .. } => name,
        }
    }

    pub fn is_dir(&self) -> bool {
        matches!(self, Node::Dir { .. })
    }

    pub fn dir(name: impl Into<String>) -> Self {
        Node::Dir {
            name: name.into(),
            children: HashMap::new(),
            expanded: false,
        }
    }
    #[inline(always)]
    pub fn expand(&mut self) -> bool {
        match self {
            Node::Dir {
                name,
                children,
                expanded,
            } => {
                *expanded = true;
                true
            }
            _ => {
                //Non - dir cannot be expanded.
                false
            }
        }
    }
    #[inline(always)]
    pub fn collapse(&mut self) -> bool {
        match self {
            Node::Dir {
                name,
                children,
                expanded,
            } => {
                *expanded = false;
                true
            }
            _ => {
                //Non - dir canot be collapsed.
                false
            }
        }
    }
    pub fn add_child(&mut self, node: Node) -> bool {
        match self {
            Node::Dir { children, .. } => {
                let node_name = node.name();
                children.insert(node_name.to_string(), node);
                true
            }
            Node::File { .. } => false,
        }
    }
}
///Thin wrapper currently, most likely want to attribute some sort of meta_data to it.
///Unsure of what exactly though. Maybe add some way of classifying the file by a custom wrapper.
#[derive(Serialize, Deserialize)]
struct Project {
    pub projects: Node,
    name: String,
    created_at: u64,
    tags: Vec<String>,
}
impl Project {
    pub fn new(name: String, tags: Vec<String>) -> Self {
        Self {
            projects: Node::Dir {
                name: name.clone(),
                children: HashMap::new(),
                expanded: false,
            },
            name,
            tags,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    pub fn load(p: impl AsRef<Path>) -> std::io::Result<Self> {
        let contents = fs::read_to_string(p)?;
        let project: Self = serde_json::from_str(&contents)?;
        Ok(project)
    }
}
