///Label all sections of the TUI that can be navigated to with keybinds with a title with the
///letter to go there in a different color than the rest of the text. Basically how btop does it.
use posty_tui::{app::AppState, key_actions::default_keymap};

fn main() -> std::io::Result<()> {
    // let keymap = default_keymap();
    // let mut app = App::new(keymap);
    // ratatui::run(|terminal| app.run(terminal))
    let keymap = default_keymap();
    let mut app = AppState::with_keymaps(keymap);
    ratatui::run(|terminal| app.run(terminal))?;
    Ok(())
}
