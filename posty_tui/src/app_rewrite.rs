use std::{collections::HashMap, hash::Hash, io};

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{DefaultTerminal, Frame, layout::Rect, widgets::Widget};

use crate::{key_actions::KeyActions, widgets::WidgetType};

pub struct App {
    //Determine what components will always be visible/required
    exit: bool,
    key_map: KeyMap,
    ///All the focused widgets should implement actionable so we can write some level of generic
    ///code for the key handling.
    focused_widget: WidgetType,
    ///This is the allocation map for bigger widgets that aren't nested.
    ///Nested widgets have their own widget_area map defined.
    ///This isn't needed but its for arbitrary cursor placement support. We need a better method
    ///for this. Currently its o(d) where d is the depth of the widget tree. This is acceptable but
    ///could probably be improved. Quad-Tree is also a possibility. Binary-search but 2D. This
    ///requires the whole allocations be known upfront however which is less flexible.
    widget_area: HashMap<WidgetType, Rect>,
}

pub type KeyMap = HashMap<KeyCode, KeyActions>;

impl App {
    ///Make an allocation table that doesn't act on raw values but rather layout/rects.
    pub fn new(key_map: KeyMap) -> Self {
        Self {
            exit: false,
            key_map,
            focused_widget: WidgetType::Empty,
            widget_area: HashMap::new(),
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
    pub fn resize() {}
}
impl App {
    pub fn move_cursor(&mut self) {}
    pub fn hit_test() {}
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) => {
                if key_event.kind == event::KeyEventKind::Press {
                    self.handle_key_events(key_event);
                }
            }
            _ => {}
        }
        Ok(())
    }
    pub fn handle_key_events(&mut self, key_event: KeyEvent) {
        let code = key_event.code;
        if let Some(action) = self.key_map.get(&code) {
            match action {
                KeyActions::LoseFocus => {
                    self.focused_widget = WidgetType::Empty;
                }
                _ => {}
            }
        }
    }
}
impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
    }
}
//fn hit_test(widget: &Widget, px: i32, py: i32) -> Option<&Widget> {
//     if !widget.bounds.contains(px, py) {
//         return None;
//     }
//     // Convert to local coordinates
//     let local_x = px - widget.bounds.x;
//     let local_y = py - widget.bounds.y;
//     // Traverse children in reverse order (topmost first)
//     for child in widget.children.iter().rev() {
//         if let Some(hit) = hit_test(child, local_x, local_y) {
//             return Some(hit);
//         }
//     }
//     // If no child hit, this widget is the target
//     Some(widget)
// }
