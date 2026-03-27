use ratatui::{
    layout::Constraint,
    style::Style,
    widgets::{Block, TableState},
};

pub struct CellValue {}

struct ITableConfig {
    ///When the user fills in something in an empty row, it'll automatically create a new row.
    extend_on_write: bool,
    max_row: usize,
    max_col: usize,
    ///By default a table contains both a title row and column.
    no_start_col: bool,
    no_start_row: bool,
}

pub struct ITableState {
    headers: Vec<String>,
    rows: Vec<Vec<CellValue>>,
    pub state: TableState,
    header_style: Style,
    cursor: (usize, usize),
    column_widths: Vec<Constraint>,
    block: Option<Block<'static>>,
}
impl ITableState {
    pub fn new(headers: impl IntoIterator<Item = impl Into<String>>) {
        let headers: Vec<String> = vec![];
    }
}
