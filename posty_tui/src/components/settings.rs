use ratatui::widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState, Widget};

pub struct Settings {}
impl Widget for &Settings {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        //Render a scrollbar
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));
    }
}
