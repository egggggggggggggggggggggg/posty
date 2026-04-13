use std::time::{Duration, Instant};

use posty::ResponseData;
use ratatui::{
    layout::{Constraint, Layout},
    widgets::Widget,
};
use reqwest::{Response, StatusCode};

use crate::{action::Actionable, tab_bar::TabBar};

enum ReseponseArea {
    Cookies,
    Body,
    Header,
}
pub struct ResponseDisplay {
    response: Option<ResponseData>,
    area: ReseponseArea,
}
impl Widget for &ResponseDisplay {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        if let Some(response) = self.response {
            //Render the status code and timestamp and request size in a bar at the top.
            let constraints = vec![Constraint::Length(1), Constraint::Fill(1)];
            let [bar, rest] = Layout::default().constraints(constraints).areas(area);

            let top_row = format!("{}", self.response.status).render(bar, buf);
            match self.area {
                ReseponseArea::Cookies => {}
                ReseponseArea::Body => {}
                ReseponseArea::Header => {}
            }
        } else {
        }
    }
}
impl Actionable for ResponseDisplay {
    fn key_event(&mut self, key: crossterm::event::KeyEvent) -> Option<AppEvent> {
        match key.code {
            _ => {}
        }
        None
    }
}
