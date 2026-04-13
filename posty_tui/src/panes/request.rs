use crossterm::event::KeyCode;
use posty::RequestData;
use ratatui::widgets::Widget;

use crate::action::Actionable;

#[derive(Debug, Clone)]
pub enum RequestArea {
    Method,
    Url,
    Param,
    Headers,
    Body,
    Auth,
}

pub struct RequestPane {
    request_data: Option<RequestData>,
    focused_area: RequestArea,
}

impl RequestPane {
    fn new() {}
}
impl Widget for &RequestPane {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        match &self.request_data {
            None => {}
            Some(r) => {
                //Render the method and URL bar;
                match self.focused_area {
                    _ => {}
                }
            }
        }
    }
}
impl Actionable for RequestPane {
    fn key_event(&mut self, key: crossterm::event::KeyEvent) -> Option<posty::executor::AppEvent> {
        match key.code {
            _ => {}
        }
        None
    }
}
