use crate::Mode;
use crate::action::Actionable;
use crate::commands::CommandPopup;
use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent, MouseEventKind};
use posty::{AppEvent, RequestData, executor::Executor};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};
use std::time::Duration;
use std::{
    io::{self},
    sync::Arc,
};
use tokio::sync::mpsc::Receiver;
const TICK_RATE: Duration = Duration::from_millis(250);
const POLL_RATE: Duration = Duration::from_millis(50);
// ----- App state -----

pub enum Pages {}

#[derive(Default)]
struct ApiRequest {}

pub struct App {
    pub exit: bool,
    pub counter: i64,
    pub tick_count: u64,
    pub current_mode: Mode,
    pub command_page: CommandPopup,
    pub editor: Editor,
    pub executor: Arc<Executor>,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            counter: 0,
            tick_count: 0,
            current_mode: Mode::default(),
            command_page: CommandPopup::default(),
            editor: Editor::default(),
            executor: Arc::new(Executor::default()),
        }
    }
    pub async fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
        app_rx: &mut Receiver<AppEvent>,
        io_rx: &mut Receiver<Event>,
    ) -> io::Result<()> {
        while !self.exit {
            draw(terminal, self)?;
            while let Some(app_event) = app_rx.recv().await {
                match app_event {
                    AppEvent::Response(r) => {
                        //Give a clone or transfer ownership over to the response pane.
                    }
                    //If the user wants to aggregate results, maybe add a dedicated task for
                    //aggregating responses and extracting the important info like status codes,
                    //etc.
                    AppEvent::ExecuteRequest(req) => {
                        self.executor.clone().spawn(req, None);
                    }
                    AppEvent::Create {
                        node_type,
                        name,
                        path,
                    } => {}
                    AppEvent::InvalidRequest(err) => {
                        //Display the error in the respective widget area.
                        //Or make a notif type message appear indicating what failed.
                    }
                    AppEvent::ChangeDisplay(fd) => {}
                    AppEvent::Tick => {}
                    AppEvent::FailedExecution(err) => {}
                    _ => {}
                }
            }
            while let Some(a) = io_rx.recv().await {
                self.handle_events(a);
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
    pub fn handle_key(&mut self, k: KeyEvent) {
        if k.is_press() {
            let key = k.code;
            match key {
                KeyCode::Char(a) => match a {
                    'q' => {
                        self.exit = true;
                    }
                    _ => {}
                },
                KeyCode::Esc => self.current_mode = Mode::Normal,
                _ => {}
            }
            //Prevent changing to other modes if not in Normal Mode,
            if let Mode::Normal = self.current_mode {
                match key {
                    KeyCode::Char('e') => self.current_mode = Mode::Execute,
                    KeyCode::Char('m') => self.current_mode = Mode::Modify,
                    KeyCode::Char('p') => self.current_mode = Mode::Performance,
                    KeyCode::Char(':') => self.current_mode = Mode::Command,
                    _ => {}
                }
            } else {
                match self.current_mode {
                    Mode::Modify => {
                        self.editor.key_event(k.clone());
                    }

                    _ => {}
                }
            }
        }
    }
    pub fn on_tick(&mut self) {}
}

pub fn draw(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    terminal.draw(|frame| {
        let area = frame.area();
        let [body, status_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1), // Workable area.
                Constraint::Length(1),
            ])
            .areas(area);
        let [panel, editor] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .areas(body);

        if let Mode::Command = app.current_mode {
            frame.render_widget(&app.command_page, area);
        }
        if let Mode::Modify = app.current_mode {
            frame.render_widget(&mut app.editor, editor);
        }

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
        frame.render_widget(status, status_area);
    })?;
    Ok(())
}
