use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use posty_tui::tabs::TabHolder;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use std::{collections::HashMap, io};

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}

//Unique widget types where there can only be one widget at a time. 
pub enum WidgetType {
    Folder(Folder),
    Input(InputField),
    TabHolder(TabHolder), 
}
impl Widget for WidgetType {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self {
            Self::Input(i) => i.render(area, buf),
            Self::Folder(f) => f.render(area, buf),
            Self::TabHolder(t) => t.render(area, buf),
        }
    }
}


///Half of what I wrote is most likely a part of the crate already. This is just for fleshing out
///the ideas so I can more easily visualize how to structure the application.
#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
    ///Stores the id of the widget where the user is currently
    focused_widget: WidgetId,
    ///Stores the widget type of the widget the user is currently
    focused_widget_type: WidgetType,
    single_widget_map: HashMap<>

}
///Uses the VonNeumann neighborhood definition. 
///If a components neighbors are two, for example where 
pub struct NeighborGraph {
    graph: 
}



impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => {
                //check if the user is currently in the tab field.
            }
            KeyCode::Right => {}
            KeyCode::Char('t') => {
                self.current_widget = 
            }
            _ => {}
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }
}
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);
        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
struct WidgetBox {
    w: usize,
    h: usize,
    row: usize,
    height: usize,
}
///This acts as a way to guarantee that all WidgetTypes have some way of fetching info on said
///widget.
trait WidgetInfo {
    ///Dimension of the widget.
    fn dims(&mut self) -> (usize, usize);
    ///How important the widget is to be full sized. If priority of this instance is higher than
    ///another LayoutHandler will prioritize rendering this at full size than the other.  
    fn priority(&mut self) -> usize;
}
///Placeholder
impl WidgetInfo for WidgetType {
    fn dims(&mut self) -> (usize, usize) {
        (0, 0)
    }
    fn priority(&mut self) -> usize {
        1
    }
}
///Folder holding contents that can be expanded to be viewed.
struct Folder {
    ///Placeholder for now, we want to be able to specify what the folder will hold.
    contents: Vec<String>,
}
impl Widget for Folder {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
    }
}
///Holds text that is put into the input field.
struct InputField {
    input: Vec<char>,
}
impl Widget for InputField {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
    }
}
impl InputField {
    ///Char_size is a parameter that specifies the input size of the input field.
    ///This is done so we can pre allocate.
    fn new(char_size: usize) -> Self {
        Self {
            input: Vec::with_capacity(char_size),
        }
    }
    fn with_capacity() {}
}
///We might wanna use this instead of hashing on an enum which has issues with retrieving the data
///associated with it. Gives a unique ID to look up for the item specified in the layout handler to
///perform stuff.
struct WidgetCreator {}

type WidgetId = usize;

///Allocates space for a given widget and is responsible for handling expanding/shrinking of the
///layout.
struct LayoutHandler {
    widget_map: HashMap<WidgetId, WidgetType>,
    ///Stores info about where the widgets are located in the terminal.
    allocations: HashMap<WidgetId, WidgetBox>,
}
impl LayoutHandler {
    fn new() -> Self {
        Self {
            widget_map: HashMap::new(),
            allocations: HashMap::new(),
        }
    }
    fn add_widget_to_layout(&mut self, widget_id: WidgetId, widget: WidgetType) {}
    ///Since ratatui already handles a lot of the layout stuff all we need to do is handle widget
    ///porportion ranking.
    fn allocate_space(&mut self) {}
    ///For a widget returns a vec of the WidgetIds of the adjacent widgets.
    fn adjacent_widgets(&mut self, widget_id: WidgetId) {}
    fn layout_test(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.area());
    }
}
pub trait Resizable {
    fn resize(&mut self);
}
