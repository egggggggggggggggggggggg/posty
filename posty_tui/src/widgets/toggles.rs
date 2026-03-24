use ratatui::{
    prelude::*,
    style::{Modifier, Style},
};

#[derive(Default)]
pub struct ToggleState {
    symbol: char,
    pub toggled: bool,
}
impl ToggleState {
    fn new(symbol: char, toggled: bool) -> Self {
        Self { symbol, toggled }
    }
    fn with_symbol(symbol: char) -> Self {
        Self {
            symbol,
            toggled: false,
        }
    }
    ///Higher level application should handle keys to toggle this.
    fn toggle(&mut self) {
        self.toggled = !self.toggled;
    }
}
struct Toggle;
impl StatefulWidget for Toggle {
    type State = ToggleState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let style = if state.toggled {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        let content = format!("[{}]", state.symbol);
        buf.set_string(area.x, area.y, content, style);
    }
}
