use std::collections::HashMap;

use ratatui::widgets::{Block, BorderType, Padding, StatefulWidget, Widget};

use crate::{
    key_actions::KeyActions,
    widgets::{
        Actionable,
        input_table::{EditableTable, EditableTableState},
    },
};

pub struct Params {
    table_state: EditableTableState,
}
impl Default for Params {
    fn default() -> Self {
        Self {
            table_state: EditableTableState::new(["Key", "Value"]),
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
        let block = Block::default()
            .border_type(BorderType::Rounded)
            .title(" Query Params ")
            .padding(Padding::new(1, 1, 1, 1));
        let inner = block.inner(area);
        block.render(area, buf);
        EditableTable::default().render(inner, buf, &mut self.table_state);
    }
}
impl Actionable for Params {
    fn key_actions(
        &mut self,
        key_actions: crate::key_actions::KeyActions,
    ) -> Option<crate::key_actions::KeyActions> {
        match key_actions {
            KeyActions::Char(ch) => match ch {
                'i' => {
                    self.
                }
                _ => {}
            },
            _ => {}
        }
        None
    }
}
