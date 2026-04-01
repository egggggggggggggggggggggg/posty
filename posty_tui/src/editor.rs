use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    symbols,
    widgets::{Block, BorderType, Borders, StatefulWidget, TableState, Tabs, Widget},
};

use crate::{
    action::Actionable,
    input_field::InputBox,
    text_editor::{TextEditor, TextEditorState},
};

#[derive(Default, Copy, Clone)]
enum Component {
    #[default]
    Parameters = 0,
    Headers = 1,
    Body = 2,
    Auth = 3,
    Endpoint = 4,
    Tabs = 5,
}
impl Component {
    fn as_tab(self) -> Option<TabTypes> {
        match self {
            Component::Parameters => Some(TabTypes::Parameters),
            Component::Headers => Some(TabTypes::Headers),
            Component::Body => Some(TabTypes::Body),
            Component::Auth => Some(TabTypes::Auth),
            _ => None,
        }
    }
}
#[derive(Default, Clone, Copy, PartialEq)]
enum TabTypes {
    #[default]
    Parameters = 0,
    Headers = 1,
    Body = 2,
    Auth = 3,
}

impl TabTypes {
    fn next(self) -> Self {
        match self {
            TabTypes::Parameters => TabTypes::Headers,
            TabTypes::Headers => TabTypes::Body,
            TabTypes::Body => TabTypes::Auth,
            TabTypes::Auth => TabTypes::Parameters, // wrap around
        }
    }

    fn back(self) -> Self {
        match self {
            TabTypes::Parameters => TabTypes::Auth, // wrap around
            TabTypes::Headers => TabTypes::Parameters,
            TabTypes::Body => TabTypes::Headers,
            TabTypes::Auth => TabTypes::Body,
        }
    }
}
#[derive(Default)]
pub struct Editor {
    selected_component: Component,
    endpoint_input: InputBox,
    body: TextEditorState,
    selected_tabtype: TabTypes,
    params: TableState,
}

impl Editor {
    fn try_switch_component(&mut self, next: Component) {
        match next.as_tab() {
            Some(tab) => {
                // Only allow if it matches current tab
                if tab == self.selected_tabtype {
                    self.selected_component = next;
                }
            }
            None => {
                // Always allow non-tab components
                self.selected_component = next;
            }
        }
    }
}
impl Widget for &mut Editor {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let [endpoint, tabs, rest] = Layout::default()
            .constraints([
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Fill(1),
            ])
            .areas(area);
        self.endpoint_input.render(endpoint, buf);
        Tabs::new(["Parameters", "Headers", "Body", "Auth"])
            .select(self.selected_tabtype as usize)
            .divider(symbols::line::VERTICAL)
            .highlight_style(Style::default().magenta().on_black().bold())
            .render(tabs, buf);
        match self.selected_tabtype {
            TabTypes::Parameters => {
                TextEditor::new()
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Cyan))
                            .border_type(BorderType::Rounded)
                            .title("Editor"),
                    )
                    .cursor_style(
                        Style::default()
                            .bg(Color::Cyan)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    )
                    .line_number_style(Style::default().fg(Color::DarkGray))
                    .current_line_style(Style::default().bg(Color::Rgb(30, 30, 50)))
                    .render(rest, buf, &mut self.body);
            }
            TabTypes::Auth => {}
            TabTypes::Headers => {}
            TabTypes::Body => {}
        }
    }
}
impl Actionable for Editor {
    fn key_event(&mut self, key: KeyEvent) {
        // Global shortcuts (always allowed, but validated)
        match (key.code, key.modifiers) {
            (KeyCode::Char('t'), KeyModifiers::CONTROL) => {
                self.try_switch_component(Component::Tabs);
                return;
            }
            (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                self.try_switch_component(Component::Parameters);
                return;
            }
            (KeyCode::Char('e'), KeyModifiers::CONTROL) => {
                self.try_switch_component(Component::Endpoint);
                return;
            }
            _ => {}
        }
        match self.selected_component {
            Component::Body => {
                self.body.key_event(key);
            }
            Component::Parameters => {
                self.body.key_event(key);
            }
            Component::Tabs => match key.code {
                KeyCode::Char('j') | KeyCode::Left => {
                    self.selected_tabtype = self.selected_tabtype.back();
                }
                KeyCode::Char('k') | KeyCode::Right => {
                    self.selected_tabtype = self.selected_tabtype.next();
                }
                _ => {}
            },

            Component::Endpoint => {
                if let KeyCode::Enter = key.code {
                    let _submitted = self.endpoint_input.content();
                }
                self.endpoint_input.key_event(key);
            }

            _ => {}
        }
    }
}

