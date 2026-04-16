use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use posty_tui::{app_1::App, run};
use ratatui::{Terminal, prelude::CrosstermBackend};

use std::io::{self, stdout};
use std::panic;

fn setup_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // Attempt to restore terminal
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
        original_hook(panic_info);
    }));
}
#[tokio::main]
async fn main() -> io::Result<()> {
    setup_panic_hook();
    ///First item is app name i think.
    let args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--path" | "-p" => {}
            "--debug" | "-d" => {
                //For this we could have the AppEvent handler, get an arg passed in  denoting
                //whether to print the events that are occuring or not. Or we could just not use
                //this.
            }
            "--host" | "-h" => {
                //Host the server for sharing request at a given ip
            }
            _ => {
                help_text();
                panic!("Unreocgnized flag ")
            }
        }
    }
    let app = App::new("")?;
    // Set up terminal
    enable_raw_mode()?;

    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    let res = run(&mut terminal).await;
    terminal.show_cursor()?;
    res
}
fn help_text() {}
