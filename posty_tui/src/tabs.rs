use ratatui::{
    layout::{Constraint, Direction, Layout, Spacing},
    style::{Modifier, Style},
    symbols::{border, merge::MergeStrategy},
    text::{Span, Text},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};
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
}

///Important api request related stuff like headers + method.
struct TabTable {}
#[derive(Default)]
struct Dropdown<T: Widget> {
    selected_item: usize,
    expanded: bool,
    items: Vec<T>,
    height: usize,
    ///These coordinates are needed as we aren't doing boxes and are arbritrarily placing the
    ///components and manually allocating space to it ourselves.
    x: usize,
    y: usize,
}
///Height is equivalent to the items in the list + 1.
impl<T: Widget> Dropdown<T> {
    fn new(items: Vec<T>, initial_item: usize, x: usize, y: usize) -> Self {
        Self {
            selected_item: initial_item,
            expanded: false,
            height: items.len() + 1,
            items,
            x,
            y,
        }
    }
    fn select(&mut self, index: usize) {
        self.selected_item = index;
    }
    fn toggle(&mut self) {
        self.expanded = !self.expanded;
    }
    fn selected_item_value(&mut self) -> &mut T {
        &mut self.items[self.selected_item]
    }
}

impl<T: Widget> Widget for Dropdown<T> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let arrow = if self.expanded { "▼" } else { "▶" };

        // When collapsed onl allocate one row; when expanded allocate one per item + header.
        let row_count = if self.expanded { self.height } else { 1 };
        let constraints = vec![Constraint::Length(1); row_count];
        let layout = Layout::new(Direction::Vertical, constraints).split(area);

        // --- Header row (always visible) ---
        // Shows the selected index and a toggle arrow. Replace `self.selected_item`
        // with a real label if you add a `labels: Vec<String>` field later.
        let header = Paragraph::new(Span::styled(
            format!(" {} Item {} ", arrow, self.selected_item),
            Style::default().add_modifier(Modifier::BOLD),
        ))
        .block(Block::default().borders(Borders::ALL));
        header.render(layout[0], buf);
        if self.expanded {
            for (i, item) in self.items.into_iter().enumerate() {
                let row_area = layout[i + 1]; // offset by 1 to skip the header row

                // Highlight the currently selected item by painting the background first.
                if i == self.selected_item {
                    Block::default()
                        .style(Style::default().bg(ratatui::style::Color::DarkGray))
                        .render(row_area, buf);
                }

                // Render the item widget itself into the same area.
                // Any widget works here: Paragraph, Gauge, custom widgets, etc.
                item.render(row_area, buf);
            }
        }
    }
}
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
