use std::path::Component;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    symbols::merge::MergeStrategy,
    widgets::{Block, BorderType, Borders, StatefulWidget, Widget},
};

use crate::{
    components::{
        ComponentType, auth::Auth, headers::Headers, params::Params, settings::Settings,
        url_bar::UrlBar,
    },
    key_actions::KeyActions,
    widgets::{Actionable, tab_bar::TabBar},
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
    tab_bar: TabBar,
    url: UrlBar,
    params: Params,
    headers: Headers,
    settings: Settings,
    auth: Auth,
    //Visible component, exlucdes UrlBar.
    focused_component: Option<ComponentType>,
    //Component that has the focus, eg the component that input gets sent to.
    active_component: Option<ComponentType>,
}

impl Widget for &mut TabView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let constraints = vec![Constraint::Max(1), Constraint::Max(5), Constraint::Fill(1)];
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Request Form ");
        let inner_area = block.inner(area);
        let layout = Layout::new(Direction::Vertical, constraints).split(inner_area);

        self.tab_bar.push(crate::widgets::tab_bar::Tab {
            title: "wefwfwfwwfewfew".to_string(),
            modified: false,
            max: 8,
        });
        self.tab_bar.render(layout[0], buf);
        let block_inner = block.inner(layout[2]);
        block.render(layout[2], buf);
        self.url.render(layout[1], buf);
        if let Some(focused) = &self.focused_component {
            match focused {
                ComponentType::Params => {
                    self.params.render(block_inner, buf);
                }
                ComponentType::Auth => {
                    self.auth.render(block_inner, buf);
                }
                ComponentType::Headers => {
                    self.headers.render(block_inner, buf);
                }
                ComponentType::Settings => {
                    self.settings.render(block_inner, buf);
                }
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
        if let Some(comp) = &self.active_component {
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
                'p' => {
                    self.active_component = Some(ComponentType::Params);
                    self.focused_component = Some(ComponentType::Params);
                }
                'a' => {
                    self.active_component = Some(ComponentType::Auth);
                    self.focused_component = Some(ComponentType::Auth);
                }
                'h' => {
                    self.active_component = Some(ComponentType::Headers);
                    self.focused_component = Some(ComponentType::Headers);
                }
                's' => {
                    self.active_component = Some(ComponentType::Settings);
                    self.focused_component = Some(ComponentType::Settings);
                }
                'u' => self.active_component = Some(ComponentType::UrlBar),
                _ => {
                    println!("This key is not supported for this component.");
                }
            },
            _ => {}
        }
        None
    }
}
