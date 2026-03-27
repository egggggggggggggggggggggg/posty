use ratatui::style::Color;
///For each widget, it should be possible to pass in a series of configuration options to customize
///them.
#[derive(Clone, Copy, Debug)]
struct WidgetTheme {
    ///Color of the border around the widget if it does exist.
    border_color: Color,
    ///Background of the widget
    bg: Color,
    ///Foreground of the widget.
    fg: Color,
    ///Series of specific component colors. The Component/Widget reads the speicfied field it needs for a
    ///given widget.
    bar: Color,

    ///Color of text.
    text_color: Color,
}

#[derive(Debug, Clone)]
pub struct TabBarTheme {
    /// Background of the whole bar (unfilled area).
    pub bar_bg: Color,
    /// Background of an inactive tab.
    pub inactive_bg: Color,
    /// Foreground of an inactive tab.
    pub inactive_fg: Color,
    /// Background of the active tab.
    pub active_bg: Color,
    /// Foreground of the active tab.
    pub active_fg: Color,
    /// Colour of the `│` divider between tabs.
    pub divider_fg: Color,
    /// Colour of the `●` modified indicator.
    pub modified_fg: Color,
    /// Colour of the `×` close indicator.
    pub close_fg: Color,
    /// Colour of the `◀` / `▶` scroll overflow arrows.
    pub scroll_indicator_fg: Color,
}
pub struct TuiTheme {
    pub term_bg: Color,
    pub border_bg: Color,
    pub text_bg: TextTheme,
    //Two colors for the bar so it can be blended.
    pub bar_range: (Color, Color),
}
pub struct TextTheme {}

impl TabBarTheme {
    /// Catppuccin Latte (light)
    pub fn catppuccin_latte() -> Self {
        Self {
            bar_bg: Color::Rgb(239, 241, 245),
            inactive_bg: Color::Rgb(239, 241, 245),
            inactive_fg: Color::Rgb(124, 127, 147),
            active_bg: Color::Rgb(220, 224, 232),
            active_fg: Color::Rgb(76, 79, 105),
            divider_fg: Color::Rgb(204, 208, 218),
            modified_fg: Color::Rgb(223, 142, 29),
            close_fg: Color::Rgb(210, 15, 57),
            scroll_indicator_fg: Color::Rgb(30, 102, 245),
        }
    }

    /// Dracula
    pub fn dracula() -> Self {
        Self {
            bar_bg: Color::Rgb(40, 42, 54),
            inactive_bg: Color::Rgb(40, 42, 54),
            inactive_fg: Color::Rgb(98, 114, 164),
            active_bg: Color::Rgb(68, 71, 90),
            active_fg: Color::Rgb(248, 248, 242),
            divider_fg: Color::Rgb(68, 71, 90),
            modified_fg: Color::Rgb(241, 250, 140),
            close_fg: Color::Rgb(255, 85, 85),
            scroll_indicator_fg: Color::Rgb(189, 147, 249),
        }
    }

    /// Nord
    pub fn nord() -> Self {
        Self {
            bar_bg: Color::Rgb(46, 52, 64),
            inactive_bg: Color::Rgb(46, 52, 64),
            inactive_fg: Color::Rgb(129, 161, 193),
            active_bg: Color::Rgb(59, 66, 82),
            active_fg: Color::Rgb(216, 222, 233),
            divider_fg: Color::Rgb(67, 76, 94),
            modified_fg: Color::Rgb(235, 203, 139),
            close_fg: Color::Rgb(191, 97, 106),
            scroll_indicator_fg: Color::Rgb(136, 192, 208),
        }
    }

    /// Gruvbox Dark
    pub fn gruvbox_dark() -> Self {
        Self {
            bar_bg: Color::Rgb(40, 40, 40),
            inactive_bg: Color::Rgb(40, 40, 40),
            inactive_fg: Color::Rgb(168, 153, 132),
            active_bg: Color::Rgb(60, 56, 54),
            active_fg: Color::Rgb(235, 219, 178),
            divider_fg: Color::Rgb(80, 73, 69),
            modified_fg: Color::Rgb(250, 189, 47),
            close_fg: Color::Rgb(251, 73, 52),
            scroll_indicator_fg: Color::Rgb(131, 165, 152),
        }
    }

    /// Tokyo Night
    pub fn tokyo_night() -> Self {
        Self {
            bar_bg: Color::Rgb(26, 27, 38),
            inactive_bg: Color::Rgb(26, 27, 38),
            inactive_fg: Color::Rgb(86, 95, 137),
            active_bg: Color::Rgb(36, 40, 59),
            active_fg: Color::Rgb(192, 202, 245),
            divider_fg: Color::Rgb(54, 58, 79),
            modified_fg: Color::Rgb(224, 175, 104),
            close_fg: Color::Rgb(247, 118, 142),
            scroll_indicator_fg: Color::Rgb(122, 162, 247),
        }
    }

    /// One Dark (Atom / VSCode style)
    pub fn one_dark() -> Self {
        Self {
            bar_bg: Color::Rgb(40, 44, 52),
            inactive_bg: Color::Rgb(40, 44, 52),
            inactive_fg: Color::Rgb(92, 99, 112),
            active_bg: Color::Rgb(56, 60, 68),
            active_fg: Color::Rgb(171, 178, 191),
            divider_fg: Color::Rgb(73, 80, 96),
            modified_fg: Color::Rgb(229, 192, 123),
            close_fg: Color::Rgb(224, 108, 117),
            scroll_indicator_fg: Color::Rgb(97, 175, 239),
        }
    }
}

impl Default for TabBarTheme {
    fn default() -> Self {
        // Catppuccin Mocha palette
        Self {
            bar_bg: Color::Rgb(24, 24, 37),                 // crust
            inactive_bg: Color::Rgb(24, 24, 37),            // crust
            inactive_fg: Color::Rgb(108, 112, 134),         // overlay0
            active_bg: Color::Rgb(49, 50, 68),              // surface0
            active_fg: Color::Rgb(205, 214, 244),           // text
            divider_fg: Color::Rgb(69, 71, 90),             // surface1
            modified_fg: Color::Rgb(249, 226, 175),         // yellow
            close_fg: Color::Rgb(243, 139, 168),            // red
            scroll_indicator_fg: Color::Rgb(137, 180, 250), // blue
        }
    }
}
