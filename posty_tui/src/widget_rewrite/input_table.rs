use ratatui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, Row, StatefulWidget, Table, TableState},
};

/// A single cell value — extend this enum if you need more types.
#[derive(Debug, Clone, PartialEq)]
pub enum CellValue {
    Text(String),
    Number(f64),
    Empty,
    ///Planned support for Dropdown, unsure what needs to be implemented first before this can work
    ///so just leaving it for now.
    Dropdown,
}

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
/// Call `.widget()` to get a render-ready `Table` + `TableState` pair.
#[derive(Debug, Clone)]
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
        // Keep selection in bounds
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
        let header_cells: Vec<Cell<'static>> = state
            .headers
            .iter()
            .map(|h| Cell::from(h.clone()).style(state.header_style))
            .collect();
        let header = Row::new(header_cells).height(1).bottom_margin(1);

        let rows: Vec<Row<'static>> = state
            .rows
            .iter()
            .map(|cells| {
                let cells: Vec<Cell<'static>> = cells
                    .iter()
                    .map(|c| Cell::from(c.as_str()).style(state.row_style))
                    .collect();
                Row::new(cells)
            })
            .collect();

        let mut table = Table::new(rows, state.column_widths.clone())
            .header(header)
            .row_highlight_style(state.selected_style)
            .highlight_symbol("▶ ");

        if let Some(block) = &state.block {
            table = table.block(block.clone());
        }
        table.render(area, buf, &mut state.state);
    }
}

