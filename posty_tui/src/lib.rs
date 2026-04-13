use std::{io, time::Duration};

use crossterm::event::{self, Event};
use posty::{IntoRequestError, RequestData, ResponseData};
use ratatui::{Terminal, prelude::CrosstermBackend};
use reqwest::Client;
use tokio::{sync::mpsc, time};

use crate::app::App;
pub mod action;
pub mod app;
pub mod card;
pub mod commands;
pub mod editor;
pub mod form;
pub mod input_field;
pub mod panes;
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
///Since both UI libraries for the GUI and TUI are immediate-mode rendering, I think this could be
///used for both so maybe move it into posty so both can access this?
pub enum AppEvent {
    Event(Event),
    Tick,
    Response(ResponseData<'static>),
    InvalidRequest(IntoRequestError),
    FailedExecution(reqwest::Error),
}
pub async fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut app = App::new();
    let (tx, mut rx) = mpsc::channel::<AppEvent>(32);
    //Task_tx will be given to app for it to tell the async task to execute a request.
    let (task_tx, mut task_rx) = mpsc::channel::<RequestData>(32);

    //Worker thread
    let worker_tx = tx.clone();
    let _worker_handle = spawn_worker(task_rx, worker_tx);

    //IO thread
    let io_tx = tx.clone();
    tokio::task::spawn_blocking(move || {
        let _ = event_loop(io_tx);
    });
    //UI thread
    app.run(terminal, &mut rx).await?;
    Ok(())
}
fn spawn_worker(
    mut task_rx: mpsc::Receiver<RequestData>,
    tick_tx: mpsc::Sender<AppEvent>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(100));
        let client = Client::new();
        loop {
            interval.tick().await;
            if tick_tx.is_closed() {
                break;
            }
            match task_rx.recv().await {
                Some(request) => match request.into_request(&client) {
                    Ok(r) => {
                        let res = client.execute(r).await.unwrap();
                        let _res_data =
                            ResponseData::extract_with_body(Duration::default(), res).await;
                    }
                    Err(e) => {
                        let _ = tick_tx.send(AppEvent::InvalidRequest(e)).await;
                    }
                },
                None => return,
            }
            if tick_tx.send(AppEvent::Tick).await.is_err() {
                break;
            }
        }
    })
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

pub mod help {
    const CARD: &'static str = "";
    const TAB: &'static str = "[j-k] [←-→] - Switch tabs";
}
