#[derive(Default)]
pub struct InputBox {
    pub left: Vec<char>,
    pub right: Vec<char>,
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
}

