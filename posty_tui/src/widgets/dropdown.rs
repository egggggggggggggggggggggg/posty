use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

use crate::{key_actions::KeyActions, widgets::Actionable};

pub struct Dropdown<T: Widget + Clone> {
    selected_item: Option<usize>,
    items: Vec<T>,
    expanded: bool,
    cursor: Option<usize>,
}
///Presets for dropdowns. Currently conflicted on whether this whole widget should be dynamic or
///just hardcode all the needed presets. Theoretically items should never change during runtime so
///it should be fine?
impl Dropdown<&'static str> {
    const PRESET_HEADERS: &[&str] = &["User-Agent", "Accept", "Accept-Encoding", "Accept-Language"];
    const PRESET_METHODS: &[&str] = &["GET", "SET", "POST", "PATCH", "DELETE"];
    pub fn preset_methods() -> Self {
        Self::with_items(Self::PRESET_METHODS.to_vec())
    }
    pub fn preset_headers() -> Self {
        Self::with_items(Self::PRESET_HEADERS.to_vec())
    }
    pub fn preset_auth() -> Self {
        Self::with_items([""].to_vec())
    }
    ///Returns the length of the longesxt item. This allows us to allocate the right amount of
    ///space for the dropdown. Just a linear search as the data isn't stored in any particular
    ///order. This only works for string types. Other T types have some level of nesting to reach
    ///the string.
    pub fn longest_item(&self) -> usize {
        let mut longest = 0;
        for i in &self.items {
            longest = longest.max(i.len());
        }
        longest
    }
}

impl<T: Widget + Clone> Dropdown<T> {
    pub fn with_items(items: Vec<T>) -> Self {
        let selected_item = if items.is_empty() { None } else { Some(0) };
        Self {
            cursor: selected_item, // cursor starts on selected
            selected_item,
            items,
            expanded: false,
        }
    }
    /// Toggle expand/collapse
    pub fn expand(&mut self) {
        self.expanded = !self.expanded;

        // when opening, sync cursor with selected
        if self.expanded {
            self.cursor = self.selected_item.or(Some(0));
        }
    }

    /// Commit cursor → selected
    pub fn commit_selection(&mut self) {
        if let Some(cursor) = self.cursor {
            self.selected_item = Some(cursor);
        }
        self.expanded = false;
    }

    /// Move cursor down
    pub fn cursor_down(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let len = self.items.len();
        self.cursor = Some(match self.cursor {
            Some(i) => (i + 1) % len,
            None => 0,
        });
    }

    /// Move cursor up
    pub fn cursor_up(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let len = self.items.len();
        self.cursor = Some(match self.cursor {
            Some(0) | None => len - 1,
            Some(i) => i - 1,
        });
    }

    /// Direct select (optional external use)
    pub fn select(&mut self, index: usize) {
        if index < self.items.len() {
            self.selected_item = Some(index);
            self.cursor = Some(index);
        }
    }

    pub fn selected(&self) -> Option<&T> {
        self.selected_item.and_then(|i| self.items.get(i))
    }

    /// Useful for rendering highlight
    pub fn cursor_item(&self) -> Option<&T> {
        self.cursor.and_then(|i| self.items.get(i))
    }
}

impl<T: Widget + Clone> Actionable for Dropdown<T> {
    fn key_actions(&mut self, key_actions: KeyActions) -> Option<KeyActions> {
        match key_actions {
            KeyActions::MoveDirection(dir) => match dir {
                crate::key_actions::Direction::Up => self.cursor_up(),
                crate::key_actions::Direction::Down => self.cursor_down(),
                _ => println!("Right and left are not allowed for nav in dropdowns"),
            },
            KeyActions::Enter => {
                if self.expanded {
                    self.commit_selection();
                    return Some(KeyActions::StateChanged);
                } else {
                    self.expand();
                }
            }
            _ => {
                println!("Action is not implemented")
            }
        }
        None
    }
}
impl<T: Widget + Clone> Widget for &Dropdown<T> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        // Calculate height
        let height = if self.expanded {
            // show all items
            self.items.len().min(area.height as usize)
        } else {
            1 // only show selected item
        };
        for y in area.top()..area.top() + height as u16 {
            for x in area.left()..area.right() {
                buf.get_mut(x, y).set_symbol(" "); // blank background
            }
        }
        if !self.expanded {
            if let Some(selected) = self.selected() {
                // Here, we assume your Widget trait has a `render_to_buf(buf, area)` method
                selected.clone().render(
                    Rect {
                        x: area.left(),
                        y: area.top(),
                        width: area.width,
                        height: 1,
                    },
                    buf,
                );
            }
            // Draw a simple ▼ arrow on the right
            if area.width > 2 {
                buf.get_mut(area.right() - 1, area.top()).set_symbol("");
            }
            return;
        }
        for (i, item) in self.items.iter().enumerate() {
            let y = area.top() + i as u16;
            if y >= area.bottom() {
                break; // prevent overflow
            }
            let mut cell_style = Style::default();
            if Some(i) == self.cursor {
                cell_style = cell_style.bg(Color::Blue).fg(Color::White);
            }
            item.clone().render(
                Rect {
                    x: area.left(),
                    y,
                    width: area.width,
                    height: 1,
                },
                buf,
            );
            // Apply highlight style
            for x in area.left()..area.right() {
                buf.get_mut(x, y).set_style(cell_style);
            }
        }
    }
}
