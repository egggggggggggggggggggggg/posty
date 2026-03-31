use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Widget};

#[derive(Default)]
pub struct InputBox {
    pub left: Vec<char>,
    pub right: Vec<char>,
    ///This is shown after the end of the right hand side.
    pub ghost_text: String,
}
impl InputBox {
    pub fn new() -> Self {
        Self::default()
    }
    /// Checks if the InputBox contains anything.
    pub fn contains_text(&self) -> bool {
        !self.left.is_empty() || !self.right.is_empty()
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
    pub fn ghost_text(&mut self, new_ghost: String) {
        self.ghost_text = new_ghost;
    }
}
impl Widget for &InputBox {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title("Input");
        block.clone().render(area, buf);
        let inner = block.inner(area);

        let content = self.content();

        let end_of_buffer = content.len();
        let style = Style::default().fg(Color::White);
        let text = Line::from(content).style(style);

        let ghost_content = self.ghost_text.clone();
        let ghost_style = Style::default().fg(Color::Gray);
        let ghost_text = Line::from(ghost_content).style(ghost_style);

        buf.set_line(inner.x, inner.y, &text, inner.width);

        let cursor_x = inner.x + self.cursor() as u16;
        let cursor_y = inner.y;

        buf.set_line(
            end_of_buffer as u16 + inner.x,
            inner.y,
            &ghost_text,
            inner.width,
        );

        if cursor_x < inner.x + inner.width {
            buf.cell_mut(Position::new(cursor_x, cursor_y))
                .expect("Cell could not be acquired for some reason")
                .set_style(Style::default().bg(Color::White).fg(Color::Black));
        }
    }
}
