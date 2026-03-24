use std::marker::PhantomData;

use ratatui::prelude::*;
use ratatui::widgets::{StatefulWidget, Widget};

use crate::{key_actions::KeyActions, widgets::Actionable};

///Any item that implements this can be used in the dropdown to be shown.
///This might be really inflexible forcing it to be &str but it'll work for now.
pub trait Displayable {
    fn display(&self) -> &str;
}
#[derive(Default)]
pub struct DropdownState<T: Widget + Clone> {
    selected_item: Option<usize>,
    items: Vec<T>,
    expanded: bool,
    cursor: Option<usize>,
}
impl<T: Widget + Clone> Actionable for DropdownState<T> {
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
impl<T: Widget + Clone> DropdownState<T> {
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
#[derive(Default)]
pub struct Dropdown<T: Widget + Clone> {
    _marker: PhantomData<T>,
}
impl<T: Widget + Clone> Dropdown<T> {}
///This is horribly written. I threw Clone in so it wouldn't freak out but its not the most
///efficient approach here. Will most likely rewrite this sometime later.
impl<T: Widget + Clone> StatefulWidget for Dropdown<T> {
    type State = DropdownState<T>;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Calculate height
        let height = if state.expanded {
            // show all items
            state.items.len().min(area.height as usize)
        } else {
            1 // only show selected item
        };

        for y in area.top()..area.top() + height as u16 {
            for x in area.left()..area.right() {
                buf.get_mut(x, y).set_symbol(" "); // blank background
            }
        }

        if !state.expanded {
            if let Some(selected) = state.selected() {
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
                buf.get_mut(area.right() - 1, area.top()).set_symbol("▼");
            }
            return;
        }

        for (i, item) in state.items.iter().enumerate() {
            let y = area.top() + i as u16;
            if y >= area.bottom() {
                break; // prevent overflow
            }
            let mut cell_style = Style::default();
            if Some(i) == state.cursor {
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
