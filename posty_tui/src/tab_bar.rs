use ratatui::{
    style::{Color, Modifier, Style},
    widgets::Widget,
};

#[derive(Default)]
pub struct Tab {
    color: Color,
    content: String,
}
impl Tab {
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    pub fn content(mut self, content: String) -> Self {
        self.content = content;
        self
    }
}

#[derive(Default)]
pub struct TabBar {
    items: Vec<Tab>,
    border_style: Style,
    tab_style: Style,
    active_style: Style,
    active: usize,
}
impl TabBar {
    pub fn with_items(items: Vec<Tab>) -> Self {
        Self {
            items,
            border_style: Style::default().fg(Color::Rgb(60, 60, 80)),
            tab_style: Style::default().add_modifier(Modifier::DIM),
            active_style: Style::default(),
            active: 0,
        }
    }
    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }
    pub fn tab_style(mut self, style: Style) -> Self {
        self.tab_style = style;
        self
    }
    pub fn active_style(mut self, style: Style) -> Self {
        self.active_style = style;
        self
    }
    pub fn next(&mut self) {
        self.active = (self.active + 1) % self.items.len();
    }
    pub fn prev(&mut self) {
        if self.active == 0 {
            self.active = self.items.len() - 1;
        } else {
            self.active -= 1;
        }
    }
}
impl Widget for TabBar {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mut x = area.x;
        for (index, item) in self.items.iter().enumerate() {
            let w = item.content.len() as u16;
            if index == self.active {
                buf.set_string(x, area.y, &item.content, self.active_style);
            } else {
                buf.set_string(x, area.y, &item.content, self.tab_style);
            }
            x += w;
            if x < area.right() {
                buf.set_string(x, area.y, " ", self.border_style);
                x += 1;
            }
        }
    }
}
