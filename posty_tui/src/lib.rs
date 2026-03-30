use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{Terminal, prelude::CrosstermBackend};
use tokio::{sync::mpsc, time};

use crate::app::{App, draw};
pub mod action;
pub mod app;
pub mod commands;
pub mod input_field;
pub mod status_bar;
#[derive(Default)]
pub enum Mode {
    #[default]
    Normal,
    Modify,
    Performance,
    Execute,
    Command,
}
enum AppEvent {
    Key(KeyCode),
    Tick,
}

pub async fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut app = App::new();

    // Channel for feeding events into the main loop
    let (tx, mut rx) = mpsc::channel::<AppEvent>(32);

    // Spawn a Tokio task that emits a Tick every 250 ms
    let tick_tx = tx.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(100));
        loop {
            interval.tick().await;
            if tick_tx.send(AppEvent::Tick).await.is_err() {
                break; // receiver dropped — app is shutting down
            }
        }
    });

    // Spawn a blocking task that reads crossterm key events and forwards them
    tokio::task::spawn_blocking(move || {
        loop {
            // poll with a short timeout so the thread stays responsive
            if event::poll(Duration::from_millis(5)).unwrap_or(false) {
                if let Ok(Event::Key(key)) = event::read() {
                    if key.kind == KeyEventKind::Press {
                        if tx.blocking_send(AppEvent::Key(key.code)).is_err() {
                            break;
                        }
                        // Let the main loop decide when to quit
                        if key.code == KeyCode::Char('q') {
                            break;
                        }
                    }
                }
            }
        }
    });

    // Custom run loop — no ratatui built-in loop involved
    loop {
        draw(terminal, &app)?;

        match rx.recv().await {
            Some(AppEvent::Tick) => app.on_tick(),
            Some(AppEvent::Key(KeyCode::Char('+'))) | Some(AppEvent::Key(KeyCode::Char('='))) => {
                app.increment()
            }
            Some(AppEvent::Key(KeyCode::Char('-'))) => app.decrement(),
            Some(AppEvent::Key(KeyCode::Char('q'))) | None => break,
            Some(AppEvent::Key(KeyCode::Esc)) => {
                app.current_mode = Mode::Normal;
            }
            Some(AppEvent::Key(KeyCode::Char('e'))) => {
                app.current_mode = Mode::Execute;
            }
            Some(AppEvent::Key(KeyCode::Char('m'))) => {
                app.current_mode = Mode::Modify;
            }
            Some(AppEvent::Key(KeyCode::Char('p'))) => {
                app.current_mode = Mode::Performance;
            }
            Some(AppEvent::Key(KeyCode::Char('c'))) => {
                app.current_mode = Mode::Command;
            }
            _ => {}
        }
    }
    Ok(())
}
