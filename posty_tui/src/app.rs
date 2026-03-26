use std::{default, io};

use crossterm::{
    event::{self, Event, KeyEvent, KeyModifiers},
    terminal,
};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    widgets::{Block, StatefulWidget, Widget},
};
use reqwest::{get, redirect::Action};

use crate::{
    components::{ComponentType, project_bar::ProjectBar, tab_view::TabView},
    key_actions::{self, KeyActions, KeyMap},
    widgets::{Actionable, WidgetType},
};
#[derive(Default)]
pub struct AppState {
    key_map: KeyMap,
    exit: bool,
    focused_widget: Option<ComponentType>,
    tab_view: TabView,
    project_bar: ProjectBar,
}
impl AppState {
    ///Dumb testing stuff, don't use this
    pub fn with_keymaps(keymap: KeyMap) -> Self {
        let mut def = Self::default();
        def.key_map = keymap;
        def
    }
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) => {
                if key_event.kind == event::KeyEventKind::Press {
                    self.handle_key_events(key_event);
                }
            }
            _ => {}
        }
        Ok(())
    }
    fn handle_key_events(&mut self, key_event: KeyEvent) {
        let code = key_event.code;
        if let Some(action) = self.key_map.get(&code) {
            if let KeyActions::Quit = action {
                self.exit = true;
                return;
            }
            match self.focused_widget {
                Some(ComponentType::Tabview) => {
                    self.tab_view.key_actions(action.clone());
                }
                _ => {
                    self.key_actions(action.clone());
                }
            }
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| {
                self.draw(frame);
            })?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}
impl Actionable for AppState {
    fn key_actions(
        &mut self,
        key_actions: crate::key_actions::KeyActions,
    ) -> Option<crate::key_actions::KeyActions> {
        match key_actions {
            KeyActions::Char(ch) => match ch {
                't' => self.focused_widget = Some(ComponentType::Tabview),
                _ => {}
            },
            _ => {}
        }
        None
    }
}
impl Widget for &mut AppState {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let vert_constraints = vec![
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Fill(1),
        ];
        let block = Block::default().title("App testing").inner(area);
        let layout = Layout::default();
        self.tab_view.render(area, buf);
    }
}
