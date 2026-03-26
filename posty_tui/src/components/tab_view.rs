use std::path::Component;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, Borders, StatefulWidget, Widget},
};

use crate::{
    components::{
        ComponentType, auth::Auth, headers::Headers, params::Params, settings::Settings,
        url_bar::UrlBar,
    },
    key_actions::KeyActions,
    widgets::{
        Actionable,
        tabs::{Tab, TabBar, TabBarState},
    },
};
#[derive(Default)]
pub enum FocusedCategory {
    #[default]
    Params,
    Headers,
    Auth,
    Settings,
}
///An actual physical tab containing the important field stuff to assemble an ApiRequest.
#[derive(Default)]
pub struct TabView {
    tab_bar: TabBarState,
    url: UrlBar,
    params: Params,
    headers: Headers,
    settings: Settings,
    auth: Auth,
    focused_component: Option<ComponentType>,
}

impl Widget for &mut TabView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let constraints = vec![Constraint::Max(1), Constraint::Max(5), Constraint::Fill(1)];
        let block = Block::default().borders(Borders::ALL).title("Request form");
        let inner_area = block.inner(area);
        let layout = Layout::new(Direction::Vertical, constraints).split(inner_area);
        self.tab_bar.push(Tab {
            title: "Test".to_string(),
            modified: false,
        });
        TabBar::default().render(area, buf, &mut self.tab_bar);
        self.url.render(layout[1], buf);
        if let Some(focused) = &self.focused_component {
            match focused {
                ComponentType::Params => {
                    self.params.render(layout[2], buf);
                }
                ComponentType::Auth => {
                    self.auth.render(layout[2], buf);
                }
                ComponentType::Headers => {
                    self.headers.render(layout[2], buf);
                }
                ComponentType::Settings => {
                    self.settings.render(layout[2], buf);
                }
                ComponentType::UrlBar => {}
                _ => {
                    panic!("This components is not part of this component. ");
                }
            }
        }
    }
}
impl Actionable for TabView {
    fn key_actions(
        &mut self,
        key_actions: crate::key_actions::KeyActions,
    ) -> Option<crate::key_actions::KeyActions> {
        if let Some(comp) = &self.focused_component {
            match comp {
                ComponentType::UrlBar => {
                    self.url.key_actions(key_actions.clone());
                    return None;
                }
                _ => {
                    println!("Key routing is not supported for this component type.")
                }
            }
        }
        match key_actions {
            KeyActions::Char(ch) => match ch {
                'p' => self.focused_component = Some(ComponentType::Params),
                'a' => self.focused_component = Some(ComponentType::Auth),
                'h' => self.focused_component = Some(ComponentType::Headers),
                's' => self.focused_component = Some(ComponentType::Settings),
                'u' => self.focused_component = Some(ComponentType::UrlBar),
                _ => {
                    println!("This key is not supported for this component.");
                }
            },
            _ => {}
        }
        None
    }
}
