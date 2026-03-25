use ratatui::widgets::Widget;

use crate::{widget_rewrite::input_box::InputBox, widgets::Actionable};

pub struct UrlBar {
    input_box: InputBox,
}
impl Widget for &UrlBar {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        self.input_box.render(area, buf);
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
