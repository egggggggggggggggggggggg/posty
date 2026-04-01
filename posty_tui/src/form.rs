use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Section {
    Parameters,
    Headers,
    Auth,
    Body,
    Endpoint,
}

impl Section {
    pub const ALL: [Section; 5] = [
        Section::Parameters,
        Section::Headers,
        Section::Auth,
        Section::Body,
        Section::Endpoint,
    ];
    pub fn next(&self) -> Self {
        match self {
            Section::Endpoint => Section::Parameters,
            Section::Parameters => Section::Headers,
            Section::Headers => Section::Auth,
            Section::Auth => Section::Body,
            Section::Body => Section::Endpoint,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Section::Endpoint => Section::Body,
            Section::Parameters => Section::Endpoint,
            Section::Headers => Section::Parameters,
            Section::Auth => Section::Headers,
            Section::Body => Section::Auth,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Section::Parameters => "PARAMS",
            Section::Headers => "HEADERS",
            Section::Auth => "AUTH",
            Section::Body => "BODY",
            Section::Endpoint => "ENDPOINT",
        }
    }

    /// A small glyph prefix shown in the tab bar.
    pub fn glyph(&self) -> &'static str {
        match self {
            Section::Parameters => "⚙",
            Section::Headers => "≡",
            Section::Auth => "★",
            Section::Body => "-",
            Section::Endpoint => "^",
        }
    }
    pub fn color(&self) -> Color {
        match self {
            Section::Parameters => Color::Cyan,
            Section::Headers => Color::Green,
            Section::Auth => Color::Yellow,
            Section::Endpoint => Color::Red,
            Section::Body => Color::Magenta,
        }
    }
    pub fn selected_bg(&self) -> Color {
        match self {
            Section::Parameters => Color::Rgb(0, 40, 50),
            Section::Headers => Color::Rgb(0, 40, 20),
            Section::Auth => Color::Rgb(50, 40, 0),
            Section::Body => Color::Rgb(100, 20, 30),
            Section::Endpoint => Color::Rgb(30, 0, 80),
        }
    }
}

#[derive(Debug, Clone)]
pub struct KvPair {
    pub key: String,
    pub value: String,
    /// Mask the value unless `show_sensitive` is true.
    pub sensitive: bool,
    /// Greyed-out / crossed-out entry (still shown but visually muted).
    pub enabled: bool,
}

impl KvPair {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            sensitive: false,
            enabled: true,
        }
    }
    pub fn sensitive(mut self) -> Self {
        self.sensitive = true;
        self
    }
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn toglge(&mut self) {
        self.enabled = !self.enabled;
    }
    ///Since KvPair is really only used with card, this helper method exists to allow passing in
    ///additional params when rendering.
    pub fn render_with_additional(
        &mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        max_key_len: usize,
        selected: bool,
        selected_bg: Color,
        show_sensitive: bool,
    ) {
        let w = area.width as usize;
        if w < 10 {
            return;
        }
        if selected {
            for x in area.x..area.right() {
                buf.cell_mut((x, area.y))
                    .expect("Invalid position")
                    .set_style(Style::default().bg(selected_bg));
            }
        }
        let accent = Color::White;
        let cursor = if selected { "▶" } else { " " };
        let cursor_style = if selected {
            Style::default()
                //Placeholder for now.
                .fg(accent)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        let (dot_char, dot_style) = if self.enabled {
            (
                "●",
                Style::default().fg(if selected {
                    accent
                } else {
                    Color::Rgb(80, 80, 80)
                }),
            )
        } else {
            ("○", Style::default().fg(Color::DarkGray))
        };
        let key_truncated = truncate(&self.key, max_key_len);
        let key_padded = format!("{:<width$}", key_truncated, width = max_key_len);
        let key_style = if !self.enabled {
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::DIM)
        } else if selected {
            Style::default().fg(accent).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let raw_value = if self.sensitive && !show_sensitive {
            "●".repeat(self.value.len().min(10))
        } else {
            self.value.clone()
        };
        let val_style = if !self.enabled {
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::DIM)
        } else if self.sensitive && !show_sensitive {
            Style::default().fg(Color::Yellow)
        } else if selected {
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };

        let prefix_len = 4; // " ▶ ● " → space + cursor(1) + space + dot(1)
        let fixed = prefix_len + max_key_len + 1; // +1 gap before dots
        let remaining = w.saturating_sub(fixed);

        // Give the value at most 60% of remaining, dots take the rest (min 3)
        let value_max = (remaining * 6 / 10).max(4);
        let value_display = truncate(&raw_value, value_max);
        let value_len = value_display.len();
        let dots_len = remaining.saturating_sub(value_len + 1).max(3); // +1 for space before value

        let dots: String = "·".repeat(dots_len);
        let dot_leader_style = Style::default().fg(Color::Rgb(45, 45, 60));
        let line = Line::from(vec![
            Span::raw(" "),
            Span::styled(cursor, cursor_style),
            Span::raw(" "),
            Span::styled(dot_char, dot_style),
            Span::raw(" "),
            Span::styled(key_padded, key_style),
            Span::raw(" "),
            Span::styled(dots, dot_leader_style),
            Span::raw(" "),
            Span::styled(value_display, val_style),
        ]);
        Paragraph::new(line).render(area, buf);
    }
}
#[inline(always)]
fn truncate(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        let cut: String = s.chars().take(max_chars.saturating_sub(1)).collect();
        format!("{cut}…")
    }
}

#[derive(Debug)]
pub struct RequestEditor {
    pub params: Vec<KvPair>,
    pub headers: Vec<KvPair>,
    pub auth: Vec<KvPair>,
    pub active_section: Section,
    pub selected_index: usize,
    pub show_sensitive: bool,
}
