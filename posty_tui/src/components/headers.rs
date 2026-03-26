use ratatui::widgets::{StatefulWidget, Widget};

use crate::widgets::{
    dropdown::Dropdown,
    input_table::{EditableTable, EditableTableState},
};
#[derive(Default)]
pub struct Headers {
    table_state: EditableTableState,
}
impl Headers {
    pub fn new() {}
}

impl Widget for &mut Headers {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let test = "a";
        Dropdown::preset_headers().render(area, buf);
        EditableTable::default().render(area, buf, &mut self.table_state);
    }
}
