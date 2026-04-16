use crossterm::event::KeyCode;
use posty::RequestData;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Widget,
};

use crate::action::Actionable;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum FocusArea {
    #[default]
    Method,
    Url,
    Panel(PanelArea),
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum PanelArea {
    Param,
    Headers,
    #[default]
    Body,
    Auth,
}
#[derive(Default)]
enum BodyFormat {
    #[default]
    Json,
    Raw,
}
#[derive(Default)]
pub struct RequestPane {
    request_data: Option<RequestData>,
    focused_area: FocusArea,
    last_panel: PanelArea,
    body_format: BodyFormat,
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
                //Render the method and URL bar
                let method = r.method();
                let url = r.url();
                let [bar, mut rest] = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(1), Constraint::Fill(1)])
                    .areas(area);
                //The format is kinda useless.
                Line::from(vec![
                    Span::styled(method, Style::default()),
                    Span::raw(format!("Url: {}", url)),
                ])
                .render(bar, buf);
                match self.last_panel {
                    PanelArea::Body => {}
                    PanelArea::Auth => {
                        if let Some(auth) = &r.auth {
                        } else {
                            //Render something that allows the user to add a new Auth type or smth.
                            //Dropdown maybe.
                        }
                    }
                    PanelArea::Headers => {
                        r.headers.iter().map(|pair| {
                            let (key, value) = pair.extract();
                            let (sym, style) = if pair.enabled {
                                ("X", Style::default())
                            } else {
                                (" ", Style::default().fg(Color::Gray))
                            };
                            rest.y += 1;
                        });
                    }
                    PanelArea::Param => {
                        r.params.iter().for_each(|pair| {
                            let (key, value) = pair.extract();
                            let (sym, style) = if pair.enabled {
                                ("X", Style::default())
                            } else {
                                (" ", Style::default().fg(Color::Gray))
                            };

                            //This is dumb as it does not check for out of bounds, but this is
                            //just to get the basic idea of how to display this stuff. Will
                            //implement the checks later.
                            rest.y += 1;
                        });
                        //Render a dropdown here with the left over space.
                    }
                }
            }
        }
    }
}
///A for adding a custom key and value.
///Enter on the dropdown to select a preset key and allow the user to enter a value.
///Auth is dropdown and doesn't have a tab.
///
impl Actionable for RequestPane {
    fn key_event(&mut self, key: crossterm::event::KeyEvent) -> Option<posty::AppEvent> {
        match key.code {
            KeyCode::Char('a') => {}
            _ => {}
        }
        None
    }
}
