///Label all sections of the TUI that can be navigated to with keybinds with a title with the
///letter to go there in a different color than the rest of the text. Basically how btop does it.
use crossterm::event::KeyCode;
use posty_tui::{
    app_rewrite::App,
    key_actions::{Direction, KeyActions},
    run,
    widgets::WidgetType,
};
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    // let keymap = default_keymap();
    // let mut app = App::new(keymap);
    // ratatui::run(|terminal| app.run(terminal))
    run()
}
///Dumb workaround for now.

