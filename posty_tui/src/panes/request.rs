use posty::RequestData;
use ratatui::widgets::Widget;

use crate::{RequestSettings, action::Actionable};

pub struct RequestPane {
    request_data: Option<RequestData>,
}
impl RequestPane {}
impl Widget for &RequestPane {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        match &self.request_data {
            None => {}
            Some(r) => {}
        }
    }
}
impl Actionable for RequestPane {
    fn key_event(&mut self, key: crossterm::event::KeyEvent) -> Option<posty::executor::AppEvent> {
        None
    }
}
