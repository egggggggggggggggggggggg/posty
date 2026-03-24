use std::{collections::HashMap, io};

use crossterm::{event::KeyCode, terminal};
use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::{
    app_rewrite::{App, AppState},
    key_actions::{Direction, KeyActions},
    widgets::{
        WidgetType,
        dropdown::{Dropdown, DropdownState},
    },
};

pub mod app;
pub mod app_rewrite;
pub mod key_actions;
pub mod tabs;
pub mod widgets;

pub fn run() -> io::Result<()> {
    let mut dropdown_state =
        DropdownState::with_items(vec!["A".to_string(), "B".to_string(), "C".to_string()]);
    let app_state = AppState {
        running: false,
        focused_widget: WidgetType::Dropdown,
        dropdown_state,
    };
    let mut app = App::new(default_keymap());
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    ratatui::run(|terminal| app.run(terminal))?;

    Ok(())
}
///Dumb workaround for now.
fn default_keymap() -> HashMap<KeyCode, KeyActions> {
    let mut map = HashMap::new();
    map.insert(KeyCode::Up, KeyActions::MoveDirection(Direction::Up));
    map.insert(KeyCode::Down, KeyActions::MoveDirection(Direction::Down));
    map.insert(KeyCode::Char('i'), KeyActions::Focus(WidgetType::InputBox));
    map.insert(KeyCode::Enter, KeyActions::Enter);
    map.insert(KeyCode::Char('q'), KeyActions::Quit);
    map
}
