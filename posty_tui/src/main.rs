use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use posty_tui::run;
use ratatui::{Terminal, prelude::CrosstermBackend};

use std::io::{self, stdout};

#[tokio::main]
async fn main() -> io::Result<()> {
    // Set up terminal
    enable_raw_mode()?;

    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    let result = run(&mut terminal).await;
    // Restore terminal regardless of outcome
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}
