use crossterm::event::{KeyCode, KeyModifiers};
use posty::AppEvent;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, StatefulWidget, Widget},
};

use crate::action::Actionable;

/// Selects which formatter [`TextEditorState::format`] will run.
///
/// Add new variants here as more formatters are needed; the match in
/// `format()` will guide you to implement them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FormatKind {
    /// No-op — returns `Ok(())` immediately without touching the buffer.
    #[default]
    None,
    /// Pretty-print the buffer as JSON using `serde_json`.
    ///
    /// The entire buffer is parsed as a single JSON value and re-serialised
    /// with two-space indentation.  Returns [`FormatError::Json`] if the
    /// content is not valid JSON.
    Json,
}

/// Error returned by [`TextEditorState::format`].
#[derive(Debug)]
pub enum FormatError {
    /// `FormatKind::Json` was requested but the buffer is not valid JSON.
    Json(serde_json::Error),
}

impl std::fmt::Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(e) => write!(f, "JSON format error: {e}"),
        }
    }
}

impl std::error::Error for FormatError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(e) => Some(e),
        }
    }
}

// ── State ─────────────────────────────────────────────────────────────────────

/// All mutable data owned by the text editor.
///
/// Store this yourself and pass a `&mut` reference to
/// [`StatefulWidget::render`].  The widget itself is stateless and cheap to
/// construct each frame.
///
/// **Backing structure**: `Vec<String>` — one `String` per logical line.
/// * Line deletion  → O(n lines) via `Vec::remove`
/// * Char insert inside a line → O(m bytes) via `String::insert` / `split_off`
/// * Random cursor movement → O(1) row + O(col) scalar traversal for byte mapping
///
/// A Rope or Piece Table would only outperform this when individual lines
/// routinely exceed tens of thousands of characters.
#[derive(Debug, Clone)]
pub struct TextEditorState {
    /// Line storage — always contains at least one entry.
    lines: Vec<String>,

    /// Cursor as `(row, col)`, both 0-indexed.
    /// `col` counts **Unicode scalar values**, not bytes.
    cursor: (usize, usize),

    /// First visible line (vertical scroll).
    scroll_offset: usize,

    /// Stored by the renderer so `move_page_up/down` work without the caller
    /// needing to track terminal dimensions.
    last_viewport_height: usize,

    /// Content-area width (excluding the line-number gutter) stored by the
    /// renderer each frame.  Used by `insert_char` when `should_wrap` is true.
    last_content_width: usize,

    /// When `true`, `insert_char` will soft-wrap the current line at a word
    /// boundary (or hard-wrap at `last_content_width` if no boundary exists)
    /// instead of letting the line grow beyond the visible content area.
    pub should_wrap: bool,
}

impl Default for TextEditorState {
    fn default() -> Self {
        Self {
            lines: vec![String::new()],
            cursor: (0, 0),
            scroll_offset: 0,
            last_viewport_height: 24,
            last_content_width: 80,
            should_wrap: false,
        }
    }
}

impl TextEditorState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn should_wrap(mut self, enable: bool) -> Self {
        self.should_wrap = enable;
        self
    }

    // ── Content ───────────────────────────────────────────────────────────────

    /// Replace the entire buffer with `content`, resetting cursor and scroll.
    pub fn load(&mut self, content: &str) {
        self.lines = content.lines().map(String::from).collect();
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }
        self.cursor = (0, 0);
        self.scroll_offset = 0;
    }

    /// Return the full buffer as a single `String` with `\n` line separators.
    pub fn content(&self) -> String {
        self.lines.join("\n")
    }

    /// Number of logical lines in the buffer.
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    // ── Formatting ────────────────────────────────────────────────────────────

    /// Format the entire buffer in-place according to `kind`.
    ///
    /// On success the buffer is replaced with the formatted text and the cursor
    /// is clamped to the nearest valid position.  On failure the buffer is
    /// **not** modified and the original error is returned.
    ///
    /// # Example
    /// ```rust,ignore
    /// state.format(FormatKind::Json)?;
    /// ```
    pub fn format(&mut self, kind: FormatKind) -> Result<(), FormatError> {
        match kind {
            // Explicit no-op so callers can pass a runtime FormatKind value
            // without wrapping every call-site in an extra `if`.
            FormatKind::None => {}

            FormatKind::Json => {
                let raw = self.content();
                // Parse into a generic Value — no concrete struct needed.
                let value: serde_json::Value =
                    serde_json::from_str(&raw).map_err(FormatError::Json)?;
                // to_string_pretty uses two-space indentation, matching `jq`.
                let pretty =
                    serde_json::to_string_pretty(&value).expect("serialising a Value never fails");
                // load() resets cursor and scroll to (0,0) cleanly.
                self.load(&pretty);
            }
        }
        Ok(())
    }

    // ── Cursor movement ───────────────────────────────────────────────────────

    /// Current cursor position `(row, col)`, both 0-indexed.
    pub fn cursor(&self) -> (usize, usize) {
        self.cursor
    }

    /// Teleport the cursor to `(row, col)`, clamping to valid bounds.
    pub fn move_to(&mut self, row: usize, col: usize) {
        let row = row.min(self.lines.len().saturating_sub(1));
        let col = col.min(self.lines[row].chars().count());
        self.cursor = (row, col);
    }

    pub fn move_up(&mut self) {
        if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
            self.clamp_col();
        }
    }

    pub fn move_down(&mut self) {
        if self.cursor.0 + 1 < self.lines.len() {
            self.cursor.0 += 1;
            self.clamp_col();
        }
    }

    /// Move left, wrapping to the end of the previous line.
    pub fn move_left(&mut self) {
        if self.cursor.1 > 0 {
            self.cursor.1 -= 1;
        } else if self.cursor.0 > 0 {
            self.cursor.0 -= 1;
            self.cursor.1 = self.lines[self.cursor.0].chars().count();
        }
    }

    /// Move right, wrapping to the start of the next line.
    pub fn move_right(&mut self) {
        let line_len = self.lines[self.cursor.0].chars().count();
        if self.cursor.1 < line_len {
            self.cursor.1 += 1;
        } else if self.cursor.0 + 1 < self.lines.len() {
            self.cursor.0 += 1;
            self.cursor.1 = 0;
        }
    }

    pub fn move_line_start(&mut self) {
        self.cursor.1 = 0;
    }

    pub fn move_line_end(&mut self) {
        self.cursor.1 = self.lines[self.cursor.0].chars().count();
    }

    /// Jump up by one viewport page.
    pub fn move_page_up(&mut self) {
        let page = self.last_viewport_height.max(1);
        self.cursor.0 = self.cursor.0.saturating_sub(page);
        self.clamp_col();
    }

    /// Jump down by one viewport page.
    pub fn move_page_down(&mut self) {
        let page = self.last_viewport_height.max(1);
        self.cursor.0 = (self.cursor.0 + page).min(self.lines.len().saturating_sub(1));
        self.clamp_col();
    }

    // ── Editing ───────────────────────────────────────────────────────────────

    /// Insert `ch` at the cursor and advance the cursor one column.
    ///
    /// When [`should_wrap`](Self::should_wrap) is `true` **and** the line
    /// would exceed [`last_content_width`] after insertion, the line is split:
    ///
    /// * **Word wrap** — searches backwards from the wrap column for the
    ///   nearest ASCII whitespace character and splits there, discarding that
    ///   space so neither fragment starts or ends with it.
    /// * **Hard wrap** — falls back to splitting exactly at `last_content_width`
    ///   when no whitespace boundary is found (e.g. a very long unbroken token).
    ///
    /// The cursor is placed correctly on the continuation line in both cases.
    pub fn insert_char(&mut self, ch: char) {
        let (row, col) = self.cursor;
        let byte_idx = self.char_to_byte(row, col);
        self.lines[row].insert(byte_idx, ch);
        self.cursor.1 += 1;

        // ── Wrap check ────────────────────────────────────────────────────────
        // Only triggered when the feature is on and the line now overflows.
        if self.should_wrap && self.last_content_width > 0 {
            let line_len = self.lines[self.cursor.0].chars().count();
            if line_len > self.last_content_width {
                self.wrap_line(self.cursor.0, self.last_content_width);
            }
        }
    }

    /// Split `lines[row]` at a word boundary near `wrap_col`, repositioning
    /// the cursor onto the correct fragment.
    ///
    /// This is a pure internal helper; it never checks `should_wrap`.
    fn wrap_line(&mut self, row: usize, wrap_col: usize) {
        // Snapshot where the cursor currently sits (one past the inserted char).
        let cursor_col_before = self.cursor.1;

        // Collect the line as chars so we can index by scalar position rather
        // than byte offset — avoids a second pass through char_indices.
        let line_chars: Vec<char> = self.lines[row].chars().collect();

        // ── Find the split point ──────────────────────────────────────────────
        // Walk backwards from wrap_col looking for ASCII whitespace.
        // The search ceiling is min(wrap_col, line_len) to avoid going OOB on
        // a line that happens to be exactly wrap_col long.
        let split_at: usize = line_chars[..wrap_col.min(line_chars.len())]
            .iter()
            .enumerate()
            .rev()
            .find(|(_, c)| c.is_ascii_whitespace())
            .map(|(i, _)| i)
            // No whitespace found in the head portion → hard-break at wrap_col.
            .unwrap_or_else(|| wrap_col.min(line_chars.len()));

        // ── Build the two fragments ───────────────────────────────────────────
        // If `split_at` points to a space, consume it so neither fragment
        // carries a leading or trailing blank; otherwise break right there.
        let is_space_boundary = line_chars
            .get(split_at)
            .map(|c| c.is_ascii_whitespace())
            .unwrap_or(false);

        let tail_start = if is_space_boundary {
            split_at + 1
        } else {
            split_at
        };

        let head: String = line_chars[..split_at].iter().collect();
        let tail: String = line_chars[tail_start..].iter().collect();

        self.lines[row] = head;
        self.lines.insert(row + 1, tail);

        // If the cursor is still inside the head, nothing changes.
        // If it's inside the tail, translate it to (row+1, offset_into_tail).
        if cursor_col_before > split_at {
            // tail_start is the first char index of the new line, so subtract
            // it to get the column position within that line.
            let offset_into_tail = cursor_col_before.saturating_sub(tail_start);
            self.cursor = (row + 1, offset_into_tail);
        }
        // else: cursor stays on `row` at its current column — no change needed.
    }

    /// Break the current line at the cursor, creating a new line below.
    pub fn insert_newline(&mut self) {
        let (row, col) = self.cursor;
        let byte_idx = self.char_to_byte(row, col);
        // split_off reuses the existing allocation for the head; the tail is a
        // new String — one allocation, no byte copying past the split point.
        let tail = self.lines[row].split_off(byte_idx);
        self.lines.insert(row + 1, tail);
        self.cursor = (row + 1, 0);
    }

    /// Delete the character **before** the cursor (backspace).
    /// Merges with the previous line when at column 0.
    pub fn backspace(&mut self) {
        let (row, col) = self.cursor;
        if col > 0 {
            let byte_idx = self.char_to_byte(row, col - 1);
            self.lines[row].remove(byte_idx);
            self.cursor.1 -= 1;
        } else if row > 0 {
            let current = self.lines.remove(row);
            let prev_len = self.lines[row - 1].chars().count();
            self.lines[row - 1].push_str(&current);
            self.cursor = (row - 1, prev_len);
        }
    }

    /// Delete the character **under** the cursor (forward delete).
    /// Merges with the next line when at end-of-line.
    pub fn delete_char(&mut self) {
        let (row, col) = self.cursor;
        let line_len = self.lines[row].chars().count();
        if col < line_len {
            let byte_idx = self.char_to_byte(row, col);
            self.lines[row].remove(byte_idx);
        } else if row + 1 < self.lines.len() {
            let next = self.lines.remove(row + 1);
            self.lines[row].push_str(&next);
        }
    }

    /// **Delete the entire current line.**
    ///
    /// Removes the row and moves the cursor to the same index (now the next
    /// line), or to the last line if this was already the last one.  When
    /// only one line exists its content is cleared instead so the buffer
    /// always retains at least one entry.
    pub fn delete_line(&mut self) {
        if self.lines.len() == 1 {
            self.lines[0].clear();
            self.cursor = (0, 0);
        } else {
            self.lines.remove(self.cursor.0);
            if self.cursor.0 >= self.lines.len() {
                self.cursor.0 = self.lines.len() - 1;
            }
            self.clamp_col();
        }
    }

    // ── Internal helpers ──────────────────────────────────────────────────────

    /// Convert a char-index column to the UTF-8 byte offset in `lines[row]`.
    fn char_to_byte(&self, row: usize, col: usize) -> usize {
        self.lines[row]
            .char_indices()
            .nth(col)
            .map(|(i, _)| i)
            .unwrap_or(self.lines[row].len())
    }

    /// Clamp `cursor.1` to the length of the current line.
    fn clamp_col(&mut self) {
        let max = self.lines[self.cursor.0].chars().count();
        if self.cursor.1 > max {
            self.cursor.1 = max;
        }
    }

    /// Called by the renderer each frame to keep scroll and layout state
    /// in sync with the actual terminal dimensions.
    pub(crate) fn sync_layout(&mut self, viewport_height: usize, content_width: usize) {
        self.last_viewport_height = viewport_height;
        self.last_content_width = content_width;

        let row = self.cursor.0;
        if row < self.scroll_offset {
            self.scroll_offset = row;
        } else if viewport_height > 0 && row >= self.scroll_offset + viewport_height {
            self.scroll_offset = row + 1 - viewport_height;
        }
    }
}

// ── Widget ────────────────────────────────────────────────────────────────────

/// A multi-line text-editing widget.
///
/// Construct it each frame and render via [`ratatui::Frame::render_stateful_widget`]
/// (or call [`StatefulWidget::render`] directly on a [`Buffer`]).
///
/// # Example
/// ```rust,ignore
/// frame.render_stateful_widget(
///     TextEditor::new()
///         .block(Block::bordered().title(" editor.rs "))
///         .cursor_style(Style::default().bg(Color::White).fg(Color::Black)),
///     area,
///     &mut editor_state,
/// );
/// ```
#[derive(Debug)]
pub struct TextEditor<'a> {
    block: Option<Block<'a>>,
    /// Base style applied to the whole widget area.
    style: Style,
    /// Style used to paint the character cell under the cursor.
    cursor_style: Style,
    /// Style for line-number gutter text.
    line_number_style: Style,
    /// Background style applied to the entire current-line row.
    current_line_style: Style,
    /// Style for the separator column between gutter and content.
    _gutter_separator_style: Style,
}

impl<'a> Default for TextEditor<'a> {
    fn default() -> Self {
        Self {
            block: None,
            style: Style::default(),
            cursor_style: Style::default()
                .bg(Color::White)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
            line_number_style: Style::default().fg(Color::DarkGray),
            current_line_style: Style::default().bg(Color::Rgb(40, 40, 40)),
            _gutter_separator_style: Style::default().fg(Color::DarkGray),
        }
    }
}

impl<'a> TextEditor<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Wrap the editor in a [`Block`] (border + optional title).
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Base style for the widget background and text.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Style for the block-cursor cell.
    pub fn cursor_style(mut self, style: Style) -> Self {
        self.cursor_style = style;
        self
    }

    /// Style for line numbers in the gutter.
    pub fn line_number_style(mut self, style: Style) -> Self {
        self.line_number_style = style;
        self
    }

    /// Background style applied to the entire row of the active line.
    pub fn current_line_style(mut self, style: Style) -> Self {
        self.current_line_style = style;
        self
    }
}

impl<'a> StatefulWidget for TextEditor<'a> {
    type State = TextEditorState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);

        let inner = match self.block {
            Some(b) => {
                let inner = b.inner(area);
                b.render(area, buf);
                inner
            }
            None => area,
        };

        if inner.width < 3 || inner.height == 0 {
            return;
        }

        // ── Gutter layout ─────────────────────────────────────────────────────
        // Width = digits needed for the last line number + "│ " (2 chars).
        let gutter_num_width = state.lines.len().to_string().len();
        let gutter_total = (gutter_num_width + 2) as u16;

        let [gutter_area, content_area] =
            Layout::horizontal([Constraint::Length(gutter_total), Constraint::Min(1)]).areas(inner);

        // Feed real dimensions back into state so wrap + page-move are accurate.
        state.sync_layout(inner.height as usize, content_area.width as usize);

        let viewport_height = inner.height as usize;
        let content_width = content_area.width as usize;
        let (cursor_row, cursor_col) = state.cursor;

        // ── Per-row rendering ─────────────────────────────────────────────────
        for vis_row in 0..viewport_height {
            let line_idx = state.scroll_offset + vis_row;
            if line_idx >= state.lines.len() {
                break;
            }

            let screen_y = inner.y + vis_row as u16;
            let is_current = line_idx == cursor_row;

            // Paint the current-line highlight across the full inner width first
            // so both the gutter and content columns share the same background.
            if is_current {
                buf.set_style(
                    Rect {
                        x: inner.x,
                        y: screen_y,
                        width: inner.width,
                        height: 1,
                    },
                    self.current_line_style,
                );
            }

            // ── Line number ───────────────────────────────────────────────────
            let num_str = format!("{:>width$}│ ", line_idx + 1, width = gutter_num_width);
            let num_style = if is_current {
                self.line_number_style
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow)
            } else {
                self.line_number_style
            };
            buf.set_string(gutter_area.x, screen_y, &num_str, num_style);

            // ── Line content with horizontal scroll ───────────────────────────
            // The active line scrolls so the cursor is always in view.
            // All other lines render from column 0.
            let h_scroll = if is_current {
                cursor_col.saturating_sub(content_width.saturating_sub(1))
            } else {
                0
            };

            let mut screen_x = content_area.x;
            let line = &state.lines[line_idx];

            for (char_idx, ch) in line.chars().enumerate().skip(h_scroll) {
                if screen_x >= content_area.x + content_area.width {
                    break;
                }
                let style = if is_current && char_idx == cursor_col {
                    self.cursor_style
                } else {
                    Style::default()
                };
                let cell = buf.cell_mut((screen_x, screen_y)).expect("Out of range");
                cell.set_char(ch);
                cell.set_style(style);
                screen_x += 1;
            }

            // Block cursor when it sits past the last character (EOL / empty line).
            if is_current && cursor_col >= line.chars().count() {
                let vis_col = (cursor_col - h_scroll) as u16;
                let cx = content_area.x + vis_col;
                if cx < content_area.x + content_area.width {
                    let cell = buf.cell_mut((cx, screen_y)).expect("Out of range");
                    cell.set_char(' ');
                    cell.set_style(self.cursor_style);
                }
            }
        }
    }
}
impl Actionable for TextEditorState {
    fn key_event(&mut self, key: crossterm::event::KeyEvent) -> Option<AppEvent> {
        match (key.code, key.modifiers) {
            (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                self.insert_char(c);
            }
            (KeyCode::Up, _) => self.move_up(),
            (KeyCode::Down, _) => self.move_down(),
            (KeyCode::Left, _) => self.move_left(),
            (KeyCode::Right, _) => self.move_right(),
            (KeyCode::Char('d'), KeyModifiers::CONTROL) => self.delete_line(),
            (KeyCode::Backspace, _) => self.backspace(),
            (KeyCode::Delete, _) => self.delete_char(),
            (KeyCode::Enter, _) => self.insert_newline(),
            (KeyCode::Char('f'), KeyModifiers::CONTROL) => {
                //If can't be formatted, ignore the error and leave as is.
                let _ = self.format(FormatKind::Json);
            }
            _ => {}
        }
        None
    }
}
