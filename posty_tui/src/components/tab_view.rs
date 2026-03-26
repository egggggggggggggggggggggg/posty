use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, Borders, Widget},
};

use crate::{
    components::{
        FocusedCategory, auth::Auth, headers::Headers, params::Params, settings::Settings,
        url_bar::UrlBar,
    },
    widgets::input_box::InputBoxState,
};

///An actual physical tab containing the important field stuff to assemble an ApiRequest.
///
struct TabView {
    url: UrlBar,
    params: Params,
    headers: Headers,
    settings: Settings,
    auth: Auth,
    focused_cateogry: FocusedCategory,
}

impl Widget for &mut TabView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let constraints = vec![Constraint::Percentage(10), Constraint::Fill(1)];
        let layout = Layout::new(Direction::Vertical, constraints);
        let block = Block::default().borders(Borders::ALL).title("Request form");
        self.url.render(area, buf);
        match self.focused_cateogry {
            FocusedCategory::Params => {
                self.params.render(area, buf);
            }
            FocusedCategory::Auth => {
                self.auth.render(area, buf);
            }
            FocusedCategory::Headers => {
                self.headers.render(area, buf);
            }
            FocusedCategory::Settings => {
                self.settings.render(area, buf);
            }
        }
    }
}
