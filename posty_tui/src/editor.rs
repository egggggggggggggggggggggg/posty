use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use posty::executor::AppEvent;
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, StatefulWidget, Widget},
};

use crate::{
    action::Actionable,
    card::Card,
    input_field::InputBox,
    tab_bar::{Tab, TabBar},
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
#[derive(Default, Clone, Copy, PartialEq)]
enum TabTypes {
    #[default]
    Parameters = 0,
    Headers = 1,
    Body = 2,
    Auth = 3,
}

impl TabTypes {
    fn as_component(self) -> Component {
        match self {
            TabTypes::Parameters => Component::Parameters,
            TabTypes::Headers => Component::Headers,
            TabTypes::Body => Component::Body,
            TabTypes::Auth => Component::Auth,
        }
    }
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
pub struct Editor {
    selected_component: Component,
    endpoint_input: InputBox,
    body: TextEditorState,
    selected_tabtype: TabTypes,
    headers: Card,
    params: Card,
    auth: Card,
}
impl Default for Editor {
    fn default() -> Self {
        let header_color = Color::Blue;
        let params_color = Color::Red;
        let auth_color = Color::Yellow;
        let card_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Green))
            .title_alignment(Alignment::Left);
        let header_card = Card::default().block(
            card_block
                .clone()
                .title(" HEADERS ")
                .border_style(Style::default().fg(header_color)),
        );
        let param_card = Card::default().block(
            card_block
                .clone()
                .title(" PARAMS ")
                .border_style(Style::default().fg(params_color)),
        );
        let auth_card = Card::default().block(
            card_block
                .clone()
                .title(" AUTH ")
                .style(Style::default().fg(auth_color)),
        );
        Self {
            selected_component: Component::default(),
            endpoint_input: InputBox::default(),
            body: TextEditorState::default().should_wrap(true),
            selected_tabtype: TabTypes::default(),
            headers: header_card,
            params: param_card,
            auth: auth_card,
        }
    }
}

impl Editor {
    fn tab_content(&self) -> Vec<Tab> {
        vec![
            Tab::default()
                .color(Color::Red)
                .content(format!("PARAMS ({})", self.params.pair_count())),
            Tab::default()
                .color(Color::Blue)
                .content(format!("HEADERS ({})", self.headers.pair_count())),
            Tab::default().color(Color::Green).content(format!("BODY")),
            Tab::default()
                .color(Color::Yellow)
                .content(format!("AUTH ({})", self.auth.pair_count())),
        ]
    }
    fn resolve_component_exit(&mut self) {
        match self.selected_component {
            Component::Endpoint => {
                self.endpoint_input.hide_cursor();
            }
            _ => {}
        }
    }
    fn extract_request(&self) {
        let headers = self.headers.collect_selected();
    }
}
impl Widget for &mut Editor {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let [endpoint, tabs, rest, help] = Layout::default()
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .areas(area);
        self.endpoint_input.render(endpoint, buf);
        TabBar::with_items(self.tab_content())
            .active(self.selected_tabtype as usize)
            .render(tabs, buf);
        match self.selected_tabtype {
            TabTypes::Parameters => {
                self.params.render(rest, buf);
            }
            TabTypes::Auth => self.auth.render(rest, buf),
            TabTypes::Headers => {
                self.headers.render(rest, buf);
            }
            TabTypes::Body => {
                TextEditor::new()
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Green))
                            .border_type(BorderType::Rounded)
                            .title(" BODY "),
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
        }
        format!("[C-t] - Focus tabs, [j-k] - Switch tabs, [C-e] - Edit focused tab")
            .render(help, buf);
    }
}

impl Actionable for Editor {
    fn key_event(&mut self, key: KeyEvent) -> Option<AppEvent> {
        // Global shortcuts (always allowed, but validated)
        match (key.code, key.modifiers) {
            (KeyCode::Char('t'), KeyModifiers::CONTROL) => {
                self.selected_component = Component::Tabs;
                self.resolve_component_exit();
                return None;
            }
            (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                self.resolve_component_exit();
                self.selected_component = self.selected_tabtype.as_component();
                if let Component::Endpoint = self.selected_component {
                    self.endpoint_input.show_curosr();
                }
                return None;
            }
            (KeyCode::Char('w'), KeyModifiers::CONTROL) => {
                //Saves the output to a specified struct format and assigns it to the current
                //project. This also triggers a file write to save the data.
                //
            }
            _ => {}
        }

        match self.selected_component {
            Component::Body => {
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
            Component::Auth => {
                self.auth.key_event(key);
            }
            Component::Headers => {
                self.headers.key_event(key);
            }
            Component::Endpoint => {
                if let KeyCode::Enter = key.code {
                    let _submitted = self.endpoint_input.content();
                }
                self.endpoint_input.key_event(key);
            }
            Component::Parameters => {
                self.params.key_event(key);
            }
        }
        None
    }
}
