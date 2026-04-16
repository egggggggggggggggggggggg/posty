use std::{io, path::Path, sync::Arc};

use crossterm::event::{Event, KeyEvent, MouseEvent};
use posty::{AppEvent, RequestData, collection::Node, executor::Executor};
use ratatui::{Terminal, prelude::CrosstermBackend};
use reqwest::Response;
use tokio::{
    select,
    sync::mpsc::{self, Receiver},
};

use crate::panes::{collections::CollectionPane, request::RequestPane, response::ResponseDisplay};
pub struct App {
    pub exit: bool,
    pub executor: Arc<Executor>,
    pub frame_rate: u32,
    pub collection: CollectionPane,
    pub response: ResponseDisplay,
    pub request: RequestPane,
}
impl App {
    pub fn new<P>(coll_path: P) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let raw_contents = std::fs::read(coll_path)?;
        let collection: Node = serde_json::from_slice(&raw_contents)?;
        Ok(Self {
            exit: false,
            frame_rate: 60,
            executor: Arc::new(Executor::default()),
            collection: CollectionPane::new(collection),
            response: ResponseDisplay::default(),
            request: RequestPane::default(),
        })
    }
    pub async fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
        app_rx: &mut Receiver<AppEvent>,
        io_rx: &mut Receiver<Event>,
    ) -> io::Result<()> {
        while !self.exit {
            draw(terminal, self)?;
            select! {
                Some(app_event) = app_rx.recv() => {
                    match app_event {
                        AppEvent::Response(r) => {
                            // handle
                        }
                        AppEvent::ExecuteRequest(req) => {
                            self.executor.clone().spawn(req, None);
                        }
                        AppEvent::Create { node_type, name, path } => {
                        }
                        AppEvent::InvalidRequest(err) => {}
                        AppEvent::ChangeDisplay(fd) => {}
                        AppEvent::Tick => {}
                        AppEvent::FailedExecution(err) => {}
                        _ => {}
                    }
                }
                Some(io_event) = io_rx.recv() => {
                    self.handle_events(io_event);
                }
            }
        }
        Ok(())
    }
    pub fn handle_events(&mut self, event: Event) {
        match event {
            Event::FocusLost => {}
            Event::Key(k) => self.handle_key(k),
            Event::Mouse(m) => self.handle_mouse(m),
            _ => {}
        }
    }
    pub fn handle_mouse(&mut self, m: MouseEvent) {}
    pub fn handle_key(&mut self, k: KeyEvent) {}
}

pub fn draw(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    terminal.draw(|frame| {
        let area = frame.area();
    })?;
    Ok(())
}
pub async fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut app = App::new("");
    let (io_tx, mut io_rx) = mpsc::unbounded_channel::<Event>();
    let (app_tx, mut app_rx) = mpsc::unbounded_channel::<AppEvent>();
    let (task_tx, mut task_rx) = mpsc::unbounded_channel::<RequestData>();
    Ok(())
}
