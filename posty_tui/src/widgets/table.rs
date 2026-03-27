use ratatui::{
    layout::Constraint,
    style::Style,
    text::Text,
    widgets::{Block, TableState, Widget},
};

use crate::widgets::{input_box::InputBox, toggle::Toggle};

pub enum AllowedTableWidgets {
    Toggle(Toggle),
    InputBox(InputBox),
    Text(String),
}
impl Widget for AllowedTableWidgets {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        match self {
            Self::Toggle(toggle) => {}
            Self::Text(text) => {}
            Self::InputBox(input_box) => {}
        }
    }
}

struct Table{
    headers: Vec<String>,
    cells: Vec<Vec<Option<AllowedTableWidgets>>>,
    header_style: Style,
    row_style: Style,
    selected_style: Style,
    column_widths: Vec<Constraint>,
    block: Option<Block<'static>>,
}
impl Table {
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


}


impl<S: Into<String>> Widget for Table<S> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        pub fn new
    }
}
