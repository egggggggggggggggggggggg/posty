use ratatui::widgets::Widget;

pub struct Auth {}
impl Widget for &Auth {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
    }
}
