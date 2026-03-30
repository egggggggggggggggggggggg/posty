use std::fmt::format;
use std::io::{self, stdout};
use std::ops::Div;
use std::panic;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::widgets::Widget;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use ratatui::{layout, text};
use tokio::sync::mpsc;
use tokio::time;

use crate::Mode;

const TICK_RATE: Duration = Duration::from_millis(250);
const POLL_RATE: Duration = Duration::from_millis(50);
// ----- App state -----

pub struct App {
    pub counter: i64,
    pub messages: Vec<String>,
    pub tick_count: u64,
    pub current_mode: Mode,
}

impl App {
    pub fn new() -> Self {
        Self {
            counter: 0,
            messages: vec!["App started! Press +/- to change counter, q to quit.".into()],
            tick_count: 0,
            current_mode: Mode::default(),
        }
    }
    pub fn increment(&mut self) {
        self.counter += 1;
        self.messages
            .push(format!("Incremented → {}", self.counter));
    }

    pub fn decrement(&mut self) {
        self.counter -= 1;
        self.messages
            .push(format!("Decremented → {}", self.counter));
    }

    pub fn on_tick(&mut self) {
        self.tick_count += 1;
    }
}

pub fn draw(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &App) -> io::Result<()> {
    terminal.draw(|frame| {
        let area = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // title
                Constraint::Length(5), // counter
                Constraint::Min(5),    // messages
                Constraint::Length(3), // footer
                Constraint::Length(1),
            ])
            .split(area);
        if let Mode::Normal = app.current_mode {}
        let (text, bg) = match app.current_mode {
            Mode::Normal => (" NORMAL ", Color::Rgb(144, 213, 255)),
            Mode::Execute => (" EXECUTE ", Color::Green),
            Mode::Modify => (" MODIFY ", Color::Magenta),
            Mode::Performance => (" PERFORMANCE ", Color::Yellow),
            Mode::Command => (" COMMAND ", Color::Cyan),
        };
        let next_bg = Color::Reset; // or whatever your footer/background is
        let status = Paragraph::new(Line::from(vec![
            // Mode block
            Span::styled(text, Style::default().fg(Color::Black).bg(bg)),
            // Powerline arrow
            Span::styled(
                "\u{e0b0}", // 
                Style::default()
                    .fg(bg) // matches previous bg
                    .bg(next_bg), // blends into next section
            ),
        ]));
        frame.render_widget(status, chunks[4]);
    })?;
    Ok(())
}
