use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use crate::{action::Actionable, form::KvPair};

///Thingie to render a section that utilizes Key Value pairs, eg params + headers. Couldve gone
///with the built in table but two colummn tables look really boring and ugly.
pub struct Card {
    ///Title of the card.
    title: String,
    ///Is the current row selected being edited? If so route key event to there.
    is_editing: bool,
    ///Pair of info.
    pairs: Vec<KvPair>,
    ///Selected KvPair,
    selected: usize,
    ///Block style
    block: Block<'static>,
    ///Color choice for the card. It'll decide a lot of the other stuff aswell.
    color: Color,
    ///Show sensitve,
    show_sensitive: bool,
}
impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}
impl Card {
    pub fn new() -> Self {
        Self {
            title: "Default".to_string(),
            is_editing: false,
            pairs: Vec::default(),
            selected: 0,
            color: Color::Green,
            block: Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Green))
                .title(Span::styled(
                    "Default",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ))
                .title_alignment(Alignment::Left)
                .title_bottom(Span::styled("[0/0]", Style::default().fg(Color::DarkGray))),
            show_sensitive: false,
        }
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
    pub fn is_editing(mut self, is_editing: bool) -> Self {
        self.is_editing = is_editing;
        self
    }
    pub fn pairs(mut self, pairs: Vec<KvPair>) -> Self {
        self.pairs = pairs;
        self
    }
    pub fn selected(mut self, selected: usize) -> Self {
        self.selected = selected;
        self
    }

    pub fn block(mut self, block: Block<'static>) -> Self {
        self.block = block;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    ///This should not be a field that can be set. Instead it should be toggled. Thus no
    ///BuilderLite.
    pub fn show_sensitive(&mut self, enable: bool) {
        self.show_sensitive = enable;
    }
    ///Returns a collection of the currently selected values.
    ///This is used for filling out an ApiRequest to be executed by the reqwest crate.
    pub fn collect_selected(&self) {}
    pub fn add_pair(&mut self, pair: KvPair) {
        self.pairs.push(pair);
    }
    pub fn create_blank_pair(&mut self) {
        self.pairs.push(KvPair::new("", ""));
    }
    pub fn pair_count(&self) -> usize {
        self.pairs.len()
    }
    pub fn current_selcted_pair(&mut self) -> &mut KvPair {
        &mut self.pairs[self.selected]
    }
}
impl Widget for &mut Card {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let total = self.pairs.len();
        let count_badge = if total > 0 {
            format!(" [{}/{}] ", self.selected, total)
        } else {
            String::from(" empty ")
        };
        let inner = self.block.inner(area);
        self.block.clone().render(area, buf);
        if self.pairs.is_empty() {
            let msg = Line::from(vec![Span::styled(
                "  ─── No entries ───",
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::ITALIC),
            )]);
            Paragraph::new(msg)
                .alignment(Alignment::Center)
                .render(inner, buf);
            return;
        }
        let list_area = Rect {
            y: inner.y + 1,
            height: inner.height.saturating_sub(1),
            ..inner
        };
        for (i, pair) in self.pairs.iter_mut().enumerate() {
            let row_y = list_area.y + i as u16;
            if row_y >= list_area.bottom() {
                break;
            }
            let row = Rect {
                x: list_area.x,
                y: row_y,
                width: list_area.width,
                height: 1,
            };
            let is_selected = self.selected == i;
            pair.render_with_additional(
                row,
                buf,
                20,
                is_selected,
                Color::Green,
                self.show_sensitive,
            );
        }
    }
}
impl Actionable for Card {
    fn key_event(&mut self, key: crossterm::event::KeyEvent) {
        match (key.code, key.modifiers) {
            (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                self.show_sensitive(true);
            }
            (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                self.show_sensitive(false);
            }
            (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                self.create_blank_pair();
            }
            (KeyCode::Up, _) => {
                if self.selected == 0 {
                    self.selected = self.pairs.len() - 1;
                } else {
                    self.selected -= 1;
                }
            }
            (KeyCode::Down, _) => {
                self.selected = (self.selected + 1) % self.pairs.len();
            }
            (KeyCode::Enter, _) => {
                if self.pairs.is_empty() {
                    return;
                }
                self.pairs[self.selected].toggle();
            }
            (KeyCode::Char(c), _) => {
                if self.is_editing {
                    let kv_pair = self.current_selcted_pair();
                }
            }

            _ => {}
        }
        if self.is_editing {
            //Route to the respective KvPair
        }
    }
}
