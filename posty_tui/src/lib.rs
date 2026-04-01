use std::{io, time::Duration};

use crossterm::event::{self, Event};
use ratatui::{Terminal, prelude::CrosstermBackend};
use tokio::{sync::mpsc, time};

use crate::app::App;
pub mod action;
pub mod app;
pub mod card;
pub mod commands;
pub mod editor;
pub mod form;
pub mod input_field;
pub mod status_bar;
pub mod tab_bar;
pub mod text_editor;
#[derive(Default)]
pub enum Mode {
    #[default]
    Normal,
    Modify,
    Performance,
    Execute,
    Command,
}
pub enum AppEvent {
    Event(Event),
    Tick,
}

pub async fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut app = App::new();
    let (tx, mut rx) = mpsc::channel::<AppEvent>(32);
    let tick_tx = tx.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(100));
        loop {
            interval.tick().await;
            if tick_tx.is_closed() {
                break;
            }
            if tick_tx.send(AppEvent::Tick).await.is_err() {
                break; // receiver dropped — app is shutting down
            }
        }
    });
    let io_tx = tx.clone();
    // Spawn a blocking task that reads crossterm key events and forwards them
    tokio::task::spawn_blocking(move || {
        let _ = event_loop(io_tx);
    });
    app.run(terminal, &mut rx).await?;
    Ok(())
}
use tokio::sync::mpsc::Sender;

fn event_loop(tx: Sender<AppEvent>) -> std::io::Result<()> {
    while !tx.is_closed() {
        if event::poll(Duration::from_millis(5))? {
            let event = event::read()?;
            if tx.blocking_send(AppEvent::Event(event)).is_err() {
                break;
            }
        }
    }
    Ok(())
}
