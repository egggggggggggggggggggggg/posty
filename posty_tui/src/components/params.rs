use std::collections::HashMap;

use ratatui::widgets::{StatefulWidget, Widget};

use crate::widget_rewrite::input_table::{EditableTable, EditableTableState};

pub struct Params {
    table_state: EditableTableState,
}
impl Default for Params {
    fn default() -> Self {
        Self {
            table_state: EditableTableState::new(["key", "value"]),
        }
    }
}
impl Params {}
///This is just to satisfy the fact that Table requires a mutable state. Might write my own
///version of Table that doesn't require state.
impl Widget for &mut Params {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        EditableTable::default().render(area, buf, &mut self.table_state);
    }
}
