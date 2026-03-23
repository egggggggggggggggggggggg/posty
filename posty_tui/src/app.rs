use std::{collections::HashSet, fmt::Octal, io};

use crate::{
    key_actions::KeyActions,
    tabs::TabArea,
    widgets::{commands::CommandBox, folder::Explorer},
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use posty::save::Node;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{self, Constraint, Direction, Layout, Spacing},
    widgets::{Block, Widget},
};

pub enum WidgetType {
    Tab,
    ///Always present but can be hidden.
    Explorer,
    ///Will take focus away from the current application
    Popup,
    ///Just a small bar that contains some message.
    Notification,
}
pub struct App {
    exit: bool,
    focused_widget: WidgetType,
    explorer: Explorer,
    tab: TabArea,
    ///Certain widget types cannot be hidden.
    hidden_tabs: HashSet<WidgetType>,
    popup_visible: bool,
    popup: CommandBox,
}
impl App {
    pub fn new(dir_location: String) -> Self {
        let tab_area = TabArea::default();
        let explorer = Explorer::default();
        Self {
            exit: false,
            focused_widget: WidgetType::Explorer,
            explorer,
            tab: tab_area,
            hidden_tabs: HashSet::new(),
            popup_visible: false,
            popup: CommandBox::default(),
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    pub fn exit(&mut self) {
        self.exit = true;
    }
}
impl App {
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) => {
                if key_event.kind == KeyEventKind::Press {
                    self.handle_key_events(key_event);
                }
            }
            _ => {}
        }
        Ok(())
    }
    ///Later rewrite note: Get the value for the key in the HashMap, if none do
    ///nothing. The value provided is an action enum. We can use this instead of just raw key
    ///matching. Since KeyCode supports serde deserialize/serialize this isn't too bad of an issue. Only issue is possible user error
    ///Maybe we could create our own config editor instead to prevent mishaps.
    fn handle_key_events(&mut self, key_event: KeyEvent) {
        let modifier_flags = key_event.modifiers;
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('e') => self.focused_widget = WidgetType::Explorer,
            KeyCode::Char('t') => self.focused_widget = WidgetType::Tab,
            KeyCode::Up => {
                if let WidgetType::Explorer = self.focused_widget {
                    self.explorer.cursor_up();
                }
            }
            KeyCode::Down => {
                if let WidgetType::Explorer = self.focused_widget {
                    self.explorer.cursor_down();
                }
            }
            KeyCode::Enter => {
                if let WidgetType::Explorer = self.focused_widget {
                    self.explorer.toggle_at_cursor();
                }
            }

            _ => {}
        }
        //Example of how we can integrate key actions for better key binding flexibility.
        let key_action = match key_event.code {
            KeyCode::Esc => KeyActions::Escape,
            KeyCode::Char('i') => KeyActions::Focus(crate::widgets::WidgetType::Input),
            _ => KeyActions::Escape,
        };
        //we then dispatch based off of whether a widget has focus.
        //This shouldnt be out here but be part of the match statement.
        //Essentially just a big state machine.
        //https://vt100.net/emu/dec_ansi_parser : example of a state machine style diagram we can
        //use.
        match self.focused_widget {
            WidgetType::Tab => {}
            _ => {}
        }
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let constraints = vec![Constraint::Percentage(10), Constraint::Percentage(90)];
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .spacing(Spacing::Overlap(1))
            .split(area);
        self.tab.render(layout[1], buf);
        self.explorer.to_list().render(layout[0], buf);
    }
}
