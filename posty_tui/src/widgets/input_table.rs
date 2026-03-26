use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, Paragraph, Row, StatefulWidget, Table, TableState, Widget},
};

/// A single cell value — extend this enum if you need more types.
#[derive(Debug, Clone, PartialEq)]
pub enum CellValue {
    Text(String),
    Number(f64),
    Empty,
    /// Planned support for Dropdown, unsure what needs to be implemented first before this can work
    /// so just leaving it for now.
    Dropdown,
}
//each column is of only one trait so it should be safe as long as we render under that assumption.

impl CellValue {
    pub fn as_str(&self) -> String {
        match self {
            CellValue::Text(s) => s.clone(),
            CellValue::Number(n) => n.to_string(),
            CellValue::Empty => String::new(),
            _ => String::new(),
        }
    }
}

impl From<&str> for CellValue {
    fn from(s: &str) -> Self {
        CellValue::Text(s.to_string())
    }
}

impl From<String> for CellValue {
    fn from(s: String) -> Self {
        CellValue::Text(s)
    }
}

impl From<f64> for CellValue {
    fn from(n: f64) -> Self {
        CellValue::Number(n)
    }
}

// ─────────────────────────────────────────────────────────────────────────────

/// Owns table data and exposes editing methods.
/// Use the `StatefulWidget` impl (via `EditableTable`) for bordered cell rendering.
/// The `.widget()` helper renders a plain ratatui `Table` without cell borders.
#[derive(Default, Debug, Clone)]
pub struct EditableTableState {
    /// Column header labels.
    headers: Vec<String>,
    /// Row-major storage: `rows[r][c]` is row *r*, column *c*.
    rows: Vec<Vec<CellValue>>,
    /// Ratatui selection state (row index).
    pub state: TableState,

    // ── styling knobs ────────────────────────────────────────────────────────
    header_style: Style,
    row_style: Style,
    selected_style: Style,
    column_widths: Vec<Constraint>,
    block: Option<Block<'static>>,
}

impl EditableTableState {
    // ── constructors ─────────────────────────────────────────────────────────

    /// Create an empty table with the given column headers.
    pub fn new(headers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let headers: Vec<String> = headers.into_iter().map(Into::into).collect();
        let col_count = headers.len();

        Self {
            headers,
            rows: Vec::new(),
            state: TableState::default(),
            header_style: Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            row_style: Style::default(),
            selected_style: Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
            column_widths: vec![Constraint::Percentage(100 / col_count as u16); col_count],
            block: None,
        }
    }

    /// Seed the table with initial rows (length must match header count).
    pub fn with_rows(
        mut self,
        rows: impl IntoIterator<Item = impl IntoIterator<Item = impl Into<CellValue>>>,
    ) -> Self {
        for row in rows {
            let cells: Vec<CellValue> = row.into_iter().map(Into::into).collect();
            assert_eq!(
                cells.len(),
                self.headers.len(),
                "Row length must match header count"
            );
            self.rows.push(cells);
        }
        self
    }

    /// Override the default equal-width column constraints.
    pub fn with_column_widths(mut self, widths: Vec<Constraint>) -> Self {
        assert_eq!(widths.len(), self.headers.len());
        self.column_widths = widths;
        self
    }

    /// Attach a `Block` (border / title) to the table.
    pub fn with_block(mut self, block: Block<'static>) -> Self {
        self.block = Some(block);
        self
    }

    // ── row editing ──────────────────────────────────────────────────────────

    /// Append a new row at the end.
    pub fn push_row(&mut self, cells: impl IntoIterator<Item = impl Into<CellValue>>) {
        let cells: Vec<CellValue> = cells.into_iter().map(Into::into).collect();
        assert_eq!(cells.len(), self.headers.len());
        self.rows.push(cells);
    }

    /// Insert a row before `index`.  Panics if `index > row_count`.
    pub fn insert_row(
        &mut self,
        index: usize,
        cells: impl IntoIterator<Item = impl Into<CellValue>>,
    ) {
        let cells: Vec<CellValue> = cells.into_iter().map(Into::into).collect();
        assert_eq!(cells.len(), self.headers.len());
        self.rows.insert(index, cells);
    }

    /// Remove the row at `index` and return it.  Panics if out of bounds.
    pub fn remove_row(&mut self, index: usize) -> Vec<CellValue> {
        let removed = self.rows.remove(index);
        if let Some(sel) = self.state.selected() {
            if sel >= self.rows.len() && !self.rows.is_empty() {
                self.state.select(Some(self.rows.len() - 1));
            } else if self.rows.is_empty() {
                self.state.select(None);
            }
        }
        removed
    }

    /// Replace an entire row.
    pub fn replace_row(
        &mut self,
        index: usize,
        cells: impl IntoIterator<Item = impl Into<CellValue>>,
    ) {
        let cells: Vec<CellValue> = cells.into_iter().map(Into::into).collect();
        assert_eq!(cells.len(), self.headers.len());
        self.rows[index] = cells;
    }

    // ── cell editing ─────────────────────────────────────────────────────────

    /// Set a single cell.
    pub fn set_cell(&mut self, row: usize, col: usize, value: impl Into<CellValue>) {
        self.rows[row][col] = value.into();
    }

    /// Get a shared reference to a cell value.
    pub fn get_cell(&self, row: usize, col: usize) -> &CellValue {
        &self.rows[row][col]
    }

    /// Get a mutable reference to a cell value.
    pub fn get_cell_mut(&mut self, row: usize, col: usize) -> &mut CellValue {
        &mut self.rows[row][col]
    }

    // ── column editing ───────────────────────────────────────────────────────

    /// Rename an existing column header.
    pub fn rename_column(&mut self, col: usize, name: impl Into<String>) {
        self.headers[col] = name.into();
    }

    /// Append an entirely new column (filled with `CellValue::Empty`).
    pub fn add_column(&mut self, header: impl Into<String>, width: Constraint) {
        self.headers.push(header.into());
        self.column_widths.push(width);
        for row in &mut self.rows {
            row.push(CellValue::Empty);
        }
    }

    /// Remove a column by index.
    pub fn remove_column(&mut self, col: usize) {
        self.headers.remove(col);
        self.column_widths.remove(col);
        for row in &mut self.rows {
            row.remove(col);
        }
    }

    // ── selection helpers ────────────────────────────────────────────────────

    /// Select the next row (wraps around).
    pub fn select_next(&mut self) {
        if self.rows.is_empty() {
            return;
        }
        let next = match self.state.selected() {
            Some(i) => (i + 1) % self.rows.len(),
            None => 0,
        };
        self.state.select(Some(next));
    }

    /// Select the previous row (wraps around).
    pub fn select_prev(&mut self) {
        if self.rows.is_empty() {
            return;
        }
        let prev = match self.state.selected() {
            Some(0) | None => self.rows.len() - 1,
            Some(i) => i - 1,
        };
        self.state.select(Some(prev));
    }

    /// Return the currently selected row's cells, if any.
    pub fn selected_row(&self) -> Option<&[CellValue]> {
        self.state.selected().map(|i| self.rows[i].as_slice())
    }

    /// Return a mutable reference to the currently selected row, if any.
    pub fn selected_row_mut(&mut self) -> Option<&mut Vec<CellValue>> {
        self.state.selected().map(|i| &mut self.rows[i])
    }

    // ── style setters ────────────────────────────────────────────────────────

    pub fn set_header_style(&mut self, style: Style) {
        self.header_style = style;
    }

    pub fn set_row_style(&mut self, style: Style) {
        self.row_style = style;
    }

    pub fn set_selected_style(&mut self, style: Style) {
        self.selected_style = style;
    }

    // ── inspection ───────────────────────────────────────────────────────────

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn col_count(&self) -> usize {
        self.headers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn headers(&self) -> &[String] {
        &self.headers
    }

    pub fn rows(&self) -> &[Vec<CellValue>] {
        &self.rows
    }

    /// Returns a plain `Table` widget **without** per-cell borders.
    /// Prefer rendering via `EditableTable` (the `StatefulWidget`) for bordered cells.
    pub fn widget(&self) -> Table<'static> {
        let header_cells: Vec<Cell<'static>> = self
            .headers
            .iter()
            .map(|h| Cell::from(h.clone()).style(self.header_style))
            .collect();
        let header = Row::new(header_cells).height(1).bottom_margin(1);

        let rows: Vec<Row<'static>> = self
            .rows
            .iter()
            .map(|cells| {
                let cells: Vec<Cell<'static>> = cells
                    .iter()
                    .map(|c| Cell::from(c.as_str()).style(self.row_style))
                    .collect();
                Row::new(cells)
            })
            .collect();

        let mut table = Table::new(rows, self.column_widths.clone())
            .header(header)
            .row_highlight_style(self.selected_style)
            .highlight_symbol("▶ ");

        if let Some(block) = &self.block {
            table = table.block(block.clone());
        }

        table
    }
}

// ─────────────────────────────────────────────────────────────────────────────

#[derive(Default)]
pub struct EditableTable;

impl StatefulWidget for EditableTable {
    type State = EditableTableState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        // ── 0. Outer block ────────────────────────────────────────────────────
        let inner = match &state.block {
            Some(block) => {
                let inner = block.inner(area);
                block.clone().render(area, buf);
                inner
            }
            None => area,
        };

        if inner.is_empty() || state.headers.is_empty() {
            return;
        }

        // ── 1. Column geometry ────────────────────────────────────────────────
        let col_areas: Vec<ratatui::prelude::Rect> =
            Layout::horizontal(state.column_widths.clone())
                .split(inner)
                .to_vec();

        // Each cell is 3 terminal rows tall: top-border + content + bottom-border.
        // Adjacent rows share one border line, so the y stride is ROW_H - 1 = 2.
        const ROW_H: u16 = 3;
        let row_count = 1 + state.rows.len(); // header row + data rows

        // ── 2. Render every cell ──────────────────────────────────────────────
        for row_idx in 0..row_count {
            let y = inner.y + row_idx as u16 * (ROW_H - 1);
            if y + ROW_H > inner.bottom() {
                break;
            }

            let is_header = row_idx == 0;
            let data_idx = row_idx.saturating_sub(1);
            let is_selected = !is_header && state.state.selected() == Some(data_idx);

            let style = if is_header {
                state.header_style
            } else if is_selected {
                state.selected_style
            } else {
                state.row_style
            };

            for (col_idx, col_area) in col_areas.iter().enumerate() {
                // Extend non-last columns by 1 so the right border of column N lands on
                // the same terminal cell as the left border of column N+1.  The later
                // draw (N+1) wins and both look identical ('│'), giving a single shared
                // border rather than the ugly double "││" you'd get from two adjacent rects.
                let cell_width = if col_idx + 1 < col_areas.len() {
                    col_areas[col_idx + 1].x - col_area.x + 1
                } else {
                    inner.right() - col_area.x
                };

                let cell_rect = ratatui::prelude::Rect {
                    x: col_area.x,
                    y,
                    width: cell_width,
                    height: ROW_H,
                };

                Block::bordered().border_style(style).render(cell_rect, buf);

                // Text goes in the true interior (left-border+1 … shared-border-1).
                let content_rect = Block::bordered().inner(cell_rect);
                let text = if is_header {
                    state.headers.get(col_idx).cloned().unwrap_or_default()
                } else {
                    state
                        .rows
                        .get(data_idx)
                        .and_then(|r| r.get(col_idx))
                        .map(CellValue::as_str)
                        .unwrap_or_default()
                };

                Paragraph::new(text).style(style).render(content_rect, buf);
            }
        }

        // ── 3. Fix junction characters ────────────────────────────────────────
        //
        // After the cell pass every crossing contains the corner glyph of whichever
        // cell was rendered last.  We overwrite those positions with the correct
        // box-drawing character for their structural role.
        //
        //   top edge + internal col   →  ┬
        //   bottom edge + internal col →  ┴
        //   internal row + left edge  →  ├
        //   internal row + right edge →  ┤
        //   internal row + internal col → ┼

        // y of the grid's bottom border
        let grid_bottom = inner.y + row_count as u16 * (ROW_H - 1);

        // Top edge: ┬ at every internal column x
        for col in col_areas.iter().skip(1) {
            if let Some(cell) = buf.cell_mut((col.x, inner.y)) {
                cell.set_symbol("┬");
            }
        }

        // Bottom edge: ┴ at every internal column x
        if grid_bottom < inner.bottom() {
            for col in col_areas.iter().skip(1) {
                if let Some(cell) = buf.cell_mut((col.x, grid_bottom)) {
                    cell.set_symbol("┴");
                }
            }
        }

        // Interior horizontal seams (between rows)
        for row_idx in 1..row_count {
            let seam_y = inner.y + row_idx as u16 * (ROW_H - 1);
            if seam_y >= inner.bottom() {
                break;
            }

            // Left edge  ├
            if let Some(cell) = buf.cell_mut((inner.x, seam_y)) {
                cell.set_symbol("├");
            }
            // Right edge  ┤
            if let Some(cell) = buf.cell_mut((inner.right() - 1, seam_y)) {
                cell.set_symbol("┤");
            }
            // Every internal column crossing  ┼
            for col in col_areas.iter().skip(1) {
                if let Some(cell) = buf.cell_mut((col.x, seam_y)) {
                    cell.set_symbol("┼");
                }
            }
        }

        // ── 4. Selection indicator ────────────────────────────────────────────
        if let Some(sel) = state.state.selected() {
            // +1 skips the header row; +1 again lands on the content line
            let indicator_y = inner.y + (sel as u16 + 1) * (ROW_H - 1) + 1;
            if indicator_y < inner.bottom() {
                if let Some(cell) = buf.cell_mut((inner.x + 1, indicator_y)) {
                    cell.set_symbol("▶");
                }
            }
        }
    }
}
