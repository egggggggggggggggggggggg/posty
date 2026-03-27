use ratatui::{
    style::{Modifier, Style},
    widgets::Widget,
};

pub struct Toggle {
    symbol: char,
    pub toggled: bool,
}
impl Toggle {
    pub fn new(symbol: char, toggled: bool) -> Self {
        Self { symbol, toggled }
    }
    pub fn with_symbol(symbol: char) -> Self {
        Self {
            symbol,
            toggled: false,
        }
    }
    fn toggle(&mut self) {
        self.toggled = !self.toggled;
    }
}
impl Widget for &Toggle {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let style = if self.toggled {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        let content = format!("[{}]", self.symbol);
        buf.set_string(area.x, area.y, content, style);
    }
}
