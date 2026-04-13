use std::time::{Duration, Instant};

use ratatui::widgets::Widget;
use reqwest::{Response, StatusCode};

use crate::tab_bar::TabBar;

pub struct ResponseDisplay {
    response_bar: ResponseBar,
    body: Body,
    header: Headers,
}
impl ResponseDisplay {
    fn populate(response: Response) {
        let cookies: Vec<reqwest::cookie::Cookie<'_>> = response.cookies().collect();
    }
}

impl Widget for &ResponseDisplay {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
    }
}

pub struct ResponseBar {
    tabs: TabBar,
    history: String,
    response_status: StatusCode,
    response_time: Duration,
    response_size: f32,
}

pub struct CookieArea {
    cookies: Vec<Cookie>,
}
impl Widget for CookieArea {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        for cookie in self.cookies {}
    }
}
pub struct Cookie {
    name: String,
    value: String,
    domain: String,
    path: String,
    exprire: String,
    http_only: bool,
    secure: bool,
}
pub struct Headers {
    keys: Vec<String>,
    value: Vec<String>,
}
enum BodyType {
    Html,
    Xml,
    Json,
    JavaScript,
    Raw,
    Hex,
    Base64,
}

pub struct Body {
    render_type: BodyType,
    ///Contains a vec of lines
    content: Vec<String>,
}
impl Widget for Body {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
    }
}
