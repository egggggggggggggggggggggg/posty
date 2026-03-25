use ratatui::{
    layout::Position,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Widget},
};

use crate::{
    key_actions::{Direction, KeyActions},
    widgets::Actionable,
};

#[derive(Default)]
pub struct InputBox {
    left: Vec<char>,
    right: Vec<char>,
}
impl InputBox {
    pub fn new() -> Self {
        Self::default()
    }
    // Insert at cursor
    pub fn insert_char(&mut self, c: char) {
        self.left.push(c);
    }
    // Backspace
    pub fn backspace(&mut self) {
        self.left.pop();
    }
    // Delete (forward)
    pub fn delete(&mut self) {
        self.right.pop();
    }
    // Move cursor left
    pub fn move_left(&mut self) {
        if let Some(c) = self.left.pop() {
            self.right.push(c);
        }
    }
    // Move cursor right
    pub fn move_right(&mut self) {
        if let Some(c) = self.right.pop() {
            self.left.push(c);
        }
    }
    // Overwrite (replace at cursor)
    pub fn overwrite_char(&mut self, c: char) {
        self.delete();
        self.insert_char(c);
    }
    // Jump cursor (rebuild gap — O(n), but rare)
    pub fn set_cursor(&mut self, pos: usize) {
        let mut full: Vec<char> = self.left.clone();
        full.extend(self.right.iter().rev());
        let pos = pos.min(full.len());
        self.left = full[..pos].to_vec();
        self.right = full[pos..].iter().rev().cloned().collect();
    }
    pub fn content(&self) -> String {
        let mut s: String = self.left.iter().collect();
        s.extend(self.right.iter().rev());
        s
    }
    ///Retains the allocated space but clears the content,
    pub fn clear(&mut self) {
        self.right.clear();
        self.left.clear();
    }
    pub fn cursor(&self) -> usize {
        self.left.len()
    }
}
impl Widget for &InputBox {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::default().borders(Borders::ALL).title("Input");
        block.clone().render(area, buf);
        let inner = block.inner(area);
        let content = self.content();
        let text = Line::from(content);
        buf.set_line(inner.x, inner.y, &text, inner.width);
        // Cursor rendering
        let cursor_x = inner.x + self.cursor() as u16;
        let cursor_y = inner.y;

        if cursor_x < inner.x + inner.width {
            buf.cell_mut(Position::new(cursor_x, cursor_y))
                .expect("Cell could not be acquired for some reason")
                .set_style(Style::default().bg(Color::White).fg(Color::Black));
        }
    }
}
impl Actionable for InputBox {
    fn key_actions(&mut self, key_actions: KeyActions) -> Option<KeyActions> {
        match key_actions {
            KeyActions::Char(ch) => {
                self.insert_char(ch);
            }
            KeyActions::MoveDirection(dir) => match dir {
                Direction::Right => {
                    self.move_right();
                }
                Direction::Left => {
                    self.move_left();
                }
                //Could tell the user that direction isn't allowed or just not do anything.
                _ => {}
            },
            _ => {
                panic!("Unrecognized command.")
            }
        }
        None
    }
}
