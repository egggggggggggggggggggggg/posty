use ratatui::widgets::Widget;

use crate::widgets::WidgetType;
#[derive(Default)]
pub struct Auth {}
impl Widget for &Auth {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
    }
}
