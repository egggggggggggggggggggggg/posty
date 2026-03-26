use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Widget},
};

use crate::{
    widget_rewrite::{dropdown::Dropdown, input_box::InputBox},
    widgets::Actionable,
};

pub struct UrlBar {
    method_dropdowns: Dropdown<&'static str>,
    input_box: InputBox,
}
impl Default for UrlBar {
    fn default() -> Self {
        Self {
            method_dropdowns: Dropdown::preset_methods(),
            input_box: InputBox::default(),
        }
    }
}
impl Widget for &UrlBar {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let constraints = vec![Constraint::Percentage(10), Constraint::Percentage(90)];
        let layout = Layout::default().constraints(constraints).split(area);
        self.method_dropdowns.render(layout[0], buf);
        self.input_box.render(layout[1], buf);
    }
}
impl Actionable for UrlBar {
    fn key_actions(
        &mut self,
        key_actions: crate::key_actions::KeyActions,
    ) -> Option<crate::key_actions::KeyActions> {
        None
    }
}
