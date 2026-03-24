use std::collections::HashMap;

use crossterm::event::KeyCode;
use posty_tui::{
    app_rewrite::App,
    key_actions::{Direction, KeyActions},
    run,
    widgets::WidgetType,
};

fn main() -> std::io::Result<()> {
    // let keymap = default_keymap();
    // let mut app = App::new(keymap);
    // ratatui::run(|terminal| app.run(terminal))
    run()
}
///Dumb workaround for now.
fn default_keymap() -> HashMap<KeyCode, KeyActions> {
    let mut map = HashMap::new();
    map.insert(KeyCode::Up, KeyActions::MoveDirection(Direction::Up));
    map.insert(KeyCode::Down, KeyActions::MoveDirection(Direction::Down));
    map.insert(KeyCode::Char('i'), KeyActions::Focus(WidgetType::InputBox));
    map.insert(KeyCode::Enter, KeyActions::Enter);
    map
}
