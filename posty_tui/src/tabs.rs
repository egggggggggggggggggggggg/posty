use ratatui::{
    layout::{Constraint, Direction, Layout, Spacing},
    style::{Modifier, Style},
    symbols::{border, merge::MergeStrategy},
    text::{Span, Text},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

use crate::widgets::dropdown::{Dropdown, DropdownState};
#[derive(Default)]
pub struct TabArea {
    open_tab: Tab,
    tab_bar: TabBar,
}
pub fn test() {}
impl TabArea {
    pub fn open_tab(&mut self, title: &str) {
        self.tab_bar.titles.push(title.to_string());
        if self.tab_bar.titles.len() == 1 {
            self.tab_bar.selected_tab = Some(0);
        }
    }
    pub fn test() {}
}
#[derive(Default)]
pub struct Tab {
    //Placeholder for now,
    content: Vec<String>,
    dropdown: Dropdown<String>,
    dropdown_state: DropdownState<String>,
}

///Important api request related stuff like headers + method.
struct TabTable {}
#[derive(Default)]
pub struct TabBar {
    selected_tab: Option<usize>,
    titles: Vec<String>,
}

impl Widget for &TabArea {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(5), Constraint::Fill(1)])
            .spacing(Spacing::Overlap(1))
            .split(area);
        self.tab_bar.render(layout[0], buf);
        self.open_tab.render(layout[1], buf);
    }
}
impl Widget for &Tab {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Block::bordered()
            .title("Tab area")
            .border_set(border::PLAIN)
            .merge_borders(MergeStrategy::Exact)
            .render(area, buf);
    }
}

impl Widget for &TabBar {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let amount = self.titles.len();
        let constraints = vec![Constraint::Fill(1); amount];
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .spacing(Spacing::Overlap(1))
            .split(area);
        let block = Block::bordered()
            .border_set(border::PLAIN)
            .merge_borders(MergeStrategy::Exact);
        for i in 0..amount {
            let text = Text::from(self.titles[i].clone());
            Paragraph::new(text)
                .block(block.clone())
                .render(layout[i], buf);
        }
    }
}
