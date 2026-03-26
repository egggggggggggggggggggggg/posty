use color_eyre::SectionExt;
use ratatui::{
    layout::{Constraint, Layout},
    style::Style,
    widgets::{Block, BorderType, Borders, Padding, Widget},
};
use reqwest::{Client, ClientBuilder};

use crate::{
    components::{ComponentType, WidgetType},
    key_actions::KeyActions,
    widgets::{Actionable, dropdown::Dropdown, input_box::InputBox},
};

pub struct UrlBar {
    method_dropdowns: Dropdown<&'static str>,
    input_box: InputBox,
    focused: ComponentType,
}
impl UrlBar {}
impl Default for UrlBar {
    fn default() -> Self {
        Self {
            method_dropdowns: Dropdown::preset_methods(),
            input_box: InputBox::default(),
            focused: ComponentType::WidgetType(WidgetType::InputBox),
        }
    }
}
impl Widget for &UrlBar {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::new()
            .title(" Url Info ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding::new(1, 1, 0, 0));
        let mut inner_area = block.inner(area);
        inner_area.height = 3;
        let dropdown_size = self.method_dropdowns.longest_item();
        let constraints = vec![Constraint::Max(dropdown_size as u16), Constraint::Fill(1)];
        block.render(area, buf);
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(constraints)
            .split(inner_area);
        self.method_dropdowns.render(layout[0], buf);
        //Render a divider for the methods and the rest of the block.
        self.input_box.render(layout[1], buf);
    }
}
impl Actionable for UrlBar {
    fn key_actions(
        &mut self,
        key_actions: crate::key_actions::KeyActions,
    ) -> Option<crate::key_actions::KeyActions> {
        match self.focused {
            ComponentType::WidgetType(WidgetType::Dropdown) => {
                self.method_dropdowns.key_actions(key_actions.clone());
                return None;
            }
            ComponentType::WidgetType(WidgetType::InputBox) => {
                self.input_box.key_actions(key_actions.clone());
                return None;
            }
            _ => {
                println!("Not supported")
            }
        }
        match key_actions {
            KeyActions::Char(ch) => match ch {
                'm' => self.focused = ComponentType::WidgetType(WidgetType::Dropdown),
                'i' => self.focused = ComponentType::WidgetType(WidgetType::InputBox),
                _ => {}
            },
            _ => {}
        }
        None
    }
}
