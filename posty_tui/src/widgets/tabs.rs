use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::StatefulWidget,
};

use crate::themes::TabBarTheme;

/// A single tab entry in the bar.
#[derive(Debug, Clone)]
pub struct Tab {
    pub title: String,
    /// When true, the indicator renders as ● (yellow) instead of × (red).
    pub modified: bool,
}

impl Tab {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            modified: false,
        }
    }
    pub fn modified(mut self) -> Self {
        self.modified = true;
        self
    }
    ///   = 1 (lead space) + title.len() + 1 (space) + 1 (indicator) + 1 (trail space)
    pub fn render_width(&self) -> u16 {
        self.title.chars().count() as u16 + 4
    }
}

/// All mutable state for the tab bar. Passed to `StatefulWidget::render`.
#[derive(Debug, Default)]
pub struct TabBarState {
    tabs: Vec<Tab>,
    /// Index of the currently active (selected) tab.
    active: usize,
    /// Index of the first tab that is currently scrolled into view.
    scroll: usize,
}

impl TabBarState {
    pub fn new() -> Self {
        Self::default()
    }
    /// Append a tab at the end.
    pub fn push(&mut self, tab: Tab) {
        self.tabs.push(tab);
    }

    /// Insert a tab at an arbitrary position (clamped to `len`).
    ///
    /// The active pointer is updated so it keeps pointing to the same tab it
    /// was pointing to before the insertion.
    pub fn insert(&mut self, at: usize, tab: Tab) {
        let at = at.min(self.tabs.len());
        self.tabs.insert(at, tab);
        // Shift active right if the new tab landed at or before it.
        if !self.tabs.is_empty() && at <= self.active && self.tabs.len() > 1 {
            self.active = (self.active + 1).min(self.tabs.len() - 1);
        }
    }

    /// Remove the tab at `at` and return it.
    ///
    /// Active index is adjusted so the neighbouring tab is selected when the
    /// active tab itself is removed.
    pub fn remove(&mut self, at: usize) -> Option<Tab> {
        if at >= self.tabs.len() {
            return None;
        }
        let tab = self.tabs.remove(at);
        if self.tabs.is_empty() {
            self.active = 0;
            self.scroll = 0;
        } else {
            // If we deleted at or before active, nudge active left (but keep ≥ 0).
            if at <= self.active && self.active > 0 {
                self.active -= 1;
            }
            // Clamp both to valid range.
            self.active = self.active.min(self.tabs.len() - 1);
            self.scroll = self.scroll.min(self.tabs.len() - 1);
        }
        Some(tab)
    }

    /// Remove and return the currently active tab.
    pub fn remove_active(&mut self) -> Option<Tab> {
        self.remove(self.active)
    }

    pub fn set_active(&mut self, idx: usize) {
        if idx < self.tabs.len() {
            self.active = idx;
        }
    }

    pub fn next(&mut self) {
        if !self.tabs.is_empty() {
            self.active = (self.active + 1) % self.tabs.len();
        }
    }

    pub fn prev(&mut self) {
        if !self.tabs.is_empty() {
            self.active = (self.active + self.tabs.len() - 1) % self.tabs.len();
        }
    }

    pub fn active_idx(&self) -> usize {
        self.active
    }

    pub fn active_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.active)
    }

    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    pub fn len(&self) -> usize {
        self.tabs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }
    /// Mutable access to the raw tab list (e.g. to toggle `modified`).
    pub fn tabs_mut(&mut self) -> &mut Vec<Tab> {
        &mut self.tabs
    }
    /// Total character width consumed by tabs[first..=last] including dividers.
    fn range_width(&self, first: usize, last: usize) -> u16 {
        if first > last || last >= self.tabs.len() {
            return 0;
        }
        let tab_sum: u16 = self.tabs[first..=last]
            .iter()
            .map(|t| t.render_width())
            .sum();
        let dividers = (last - first) as u16; // one `│` between consecutive tabs
        tab_sum + dividers
    }

    /// Adjust `self.scroll` so the active tab is visible within `avail_width`
    /// characters.  The renderer calls this every frame before drawing.
    pub(crate) fn ensure_active_visible(&mut self, avail_width: u16) {
        if self.tabs.is_empty() {
            self.scroll = 0;
            return;
        }

        // 1. Scroll cannot be past active.
        if self.scroll > self.active {
            self.scroll = self.active;
        }

        // 2. Advance scroll until active fits inside avail_width.
        //    We stop if scroll would pass active (safety valve).
        while self.scroll < self.active {
            if self.range_width(self.scroll, self.active) <= avail_width {
                break;
            }
            self.scroll += 1;
        }
    }

    /// How many tabs fit starting at `scroll` within `avail_width`.
    pub(crate) fn count_visible(&self, avail_width: u16) -> usize {
        if self.tabs.is_empty() {
            return 0;
        }
        let mut count = 0usize;
        for last in self.scroll..self.tabs.len() {
            if self.range_width(self.scroll, last) > avail_width {
                break;
            }
            count = last - self.scroll + 1;
        }
        count
    }
}

/// Bufferline-style tab bar widget.
///
/// Implements [`StatefulWidget`]; pair it with a [`TabBarState`].
///
/// ## Example
/// ```rust
/// frame.render_stateful_widget(TabBar::new(), tab_area, &mut app.tab_state);
/// ```
#[derive(Default)]
pub struct TabBar {
    theme: TabBarTheme,
}

impl TabBar {
    pub fn new() -> Self {
        Self::default()
    }

    /// Override the colour theme.
    pub fn theme(mut self, theme: TabBarTheme) -> Self {
        self.theme = theme;
        self
    }
}

impl StatefulWidget for TabBar {
    type State = TabBarState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if area.height == 0 || area.width == 0 {
            return;
        }

        let y = area.top();
        let t = &self.theme;
        let bar_style = Style::default().bg(t.bar_bg);

        // ── 1. Fill entire row with bar background ───────────────────────────
        for x in area.left()..area.right() {
            buf.get_mut(x, y).set_symbol(" ").set_style(bar_style);
        }

        if state.is_empty() {
            return;
        }

        // ── 2. Determine overflow, reserving 2 cols each side when needed ────
        //
        // We do two passes: first a coarse pass ignoring indicators, then a
        // refined pass once we know which side indicators are required.

        // Coarse pass — no indicator columns reserved yet.
        state.ensure_active_visible(area.width);

        let has_left_overflow = state.scroll > 0;
        let left_reserve: u16 = if has_left_overflow { 2 } else { 0 };

        // Compute how many tabs fit on the right side with left reserve applied.
        let avail_right = area.width.saturating_sub(left_reserve);
        let visible = state.count_visible(avail_right);
        let has_right_overflow = state.scroll + visible < state.tabs.len();
        let right_reserve: u16 = if has_right_overflow { 2 } else { 0 };

        // Final available width for tabs.
        let tab_area_width = area.width.saturating_sub(left_reserve + right_reserve);

        // Recalculate after reserving right, adjusting scroll once more.
        state.ensure_active_visible(tab_area_width);
        let visible = state.count_visible(tab_area_width);
        let has_left_overflow = state.scroll > 0;
        let has_right_overflow = state.scroll + visible < state.tabs.len();

        // ── 3. Draw left overflow arrow ──────────────────────────────────────
        let mut x = area.left();
        if has_left_overflow {
            buf.get_mut(x, y)
                .set_symbol("◀")
                .set_style(Style::default().fg(t.scroll_indicator_fg).bg(t.bar_bg));
            x += 1;
            buf.get_mut(x, y).set_symbol(" ").set_style(bar_style);
            x += 1;
        }

        // Right boundary (exclusive) that tabs may occupy.
        let right_tab_boundary = area.right() - if has_right_overflow { 2 } else { 0 };

        // ── 4. Draw each visible tab ─────────────────────────────────────────
        for (vis_i, abs_i) in (state.scroll..state.scroll + visible).enumerate() {
            let tab = &state.tabs[abs_i];
            let is_active = abs_i == state.active;

            let tab_bg = if is_active {
                t.active_bg
            } else {
                t.inactive_bg
            };
            let tab_fg = if is_active {
                t.active_fg
            } else {
                t.inactive_fg
            };
            let base_style = if is_active {
                Style::default()
                    .fg(tab_fg)
                    .bg(tab_bg)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(tab_fg).bg(tab_bg)
            };

            // ── Divider between consecutive tabs ────────────────────────────
            if vis_i > 0 && x < right_tab_boundary {
                // The active tab gets a slightly brighter divider on its sides.
                let div_style = Style::default().fg(t.divider_fg).bg(t.inactive_bg);
                buf.get_mut(x, y).set_symbol("│").set_style(div_style);
                x += 1;
            }

            // ── Leading space ───────────────────────────────────────────────
            if x < right_tab_boundary {
                buf.get_mut(x, y).set_symbol(" ").set_style(base_style);
                x += 1;
            }

            // ── Title characters ────────────────────────────────────────────
            for ch in tab.title.chars() {
                if x >= right_tab_boundary {
                    break;
                }
                let mut cell = buf.get_mut(x, y);
                cell.set_symbol(&ch.to_string());
                cell.set_style(base_style);
                x += 1;
            }

            // ── Space before indicator ───────────────────────────────────────
            if x < right_tab_boundary {
                buf.get_mut(x, y).set_symbol(" ").set_style(base_style);
                x += 1;
            }

            // ── Modified / close indicator ──────────────────────────────────
            if x < right_tab_boundary {
                let (symbol, ind_fg) = if tab.modified {
                    ("●", t.modified_fg)
                } else {
                    ("×", t.close_fg)
                };
                buf.get_mut(x, y)
                    .set_symbol(symbol)
                    .set_style(Style::default().fg(ind_fg).bg(tab_bg));
                x += 1;
            }

            // ── Trailing space ──────────────────────────────────────────────
            if x < right_tab_boundary {
                buf.get_mut(x, y).set_symbol(" ").set_style(base_style);
                x += 1;
            }
        }

        // ── 5. Draw right overflow arrow ─────────────────────────────────────
        if has_right_overflow {
            let arrow_x = area.right() - 1;
            buf.get_mut(arrow_x - 1, y)
                .set_symbol(" ")
                .set_style(bar_style);
            buf.get_mut(arrow_x, y)
                .set_symbol("▶")
                .set_style(Style::default().fg(t.scroll_indicator_fg).bg(t.bar_bg));
        }
    }
}
