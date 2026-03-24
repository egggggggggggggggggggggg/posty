use std::collections::HashMap;

use posty::save::ApiRequest;
use ratatui::{
    layout::{self, Constraint, Layout, Rect, Spacing},
    widgets::{Block, Paragraph, StatefulWidget, Table, TableState},
};

use crate::{
    key_actions::KeyActions,
    widgets::{
        Actionable, WidgetType,
        dropdown::{Dropdown, DropdownState},
        input_box::{InputBox, InputBoxState},
    },
};
#[derive(Default)]
enum Method {
    #[default]
    Get,
    Post,
    Set,
    Patch,
    Put,
    Delete,
}
#[derive(Default)]
enum Category {
    #[default]
    Params,
    Header,
    Authorization,
    Body,
    Scripts,
    Settings,
}
impl Category {
    fn next(self) -> Self {
        match self {
            Category::Params => Category::Body,
            Category::Body => Category::Header,
            Category::Header => Category::Authorization,
            Category::Authorization => Category::Scripts,
            Category::Scripts => Category::Settings,
            Category::Settings => Category::Params,
        }
    }
}
#[derive(Default)]
pub enum TabSection {
    Category,
    Method,
    UrlInput,
    CategoryInput,
    #[default]
    ///No cursor visible,
    None,
}

///Fat struct that should probably be split. The majority of the fields revolve around how we can
///transfer the user across widgets to edit/modify the widget state.
///Do not call the default method as it'll just be a blank tab.
pub struct TabState {
    selected_method: Method,
    selected_category: Category,
    focused_section: TabSection,
    pub api_request: ApiRequest,
    focused_widget: WidgetType,
    pub input_box: InputBoxState,
    pub param_table: TableState,
    pub method_dropdown: DropdownState<String>,
}

impl TabState {
    pub fn extractt(&mut self) -> &ApiRequest {
        &self.api_request
    }
    ///This does not handle losing focus. That is handled in the key_action function.
    pub fn change_focus(&mut self, new_wiget: WidgetType) {}
    ///The focus of a widget changedd by the postion of the cursor placement.
    pub fn cursor_focus(&mut self, x: usize, y: usize) {}
}
#[derive(Default)]
pub struct Tab;
impl StatefulWidget for Tab {
    type State = TabState;
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let constraints = vec![Constraint::Percentage(70), Constraint::Fill(1)];
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(constraints)
            .spacing(Spacing::Overlap(1))
            .split(area);
        let top_section = layout[0];
        let top_constraints = vec![Constraint::Percentage(10), Constraint::Fill(1)];
        let top_layout = Layout::default()
            .direction(layout::Direction::Horizontal)
            .constraints(top_constraints)
            .spacing(Spacing::Overlap(1))
            .split(top_section);
        Dropdown::default().render(top_layout[0], buf, &mut state.method_dropdown);
        InputBox::default().render(top_layout[1], buf, &mut state.input_box);
        Table::default().render(layout[1], buf, &mut state.param_table);
    }
}
impl Actionable for TabState {
    fn key_actions(&mut self, action: KeyActions) -> Option<KeyActions> {
        match action {
            KeyActions::Escape => return Some(KeyActions::LoseFocus),
            KeyActions::Focus(widget) => self.change_focus(widget),
            KeyActions::Char(a) => {
                if let WidgetType::InputBox = self.focused_widget {
                    self.input_box.insert_char(a);
                }
            }
            _ => {}
        }
        None
    }
}
//Denotes the current category focused within the Tab.
///Holds all the info required to make an ApiRequest

enum ConversionError {
    Invalid,
}
enum AuthType {}
///A table you can edit wtih a visual cursor to show.
struct InputTableState {}
impl Actionable for TableState {
    fn key_actions(&mut self, key_actions: KeyActions) -> Option<KeyActions> {
        None
    }
}
struct CentralizedState {
    url: InputBoxState,
    params: HashMap<String, String>,
    authorization: AuthType,
    headers: HashMap<String, String>,
    body: InputBoxState,
    settings: HashMap<SettingOptions, bool>,
    focused_section: TabCategory,
}
impl CentralizedState {
    fn try_into_request() -> Result<ApiRequest, ConversionError> {
        Err(ConversionError::Invalid)
    }
}
impl Actionable for CentralizedState {
    fn key_actions(&mut self, key_actions: KeyActions) -> Option<KeyActions> {
        None
    }
}

#[derive(Clone, Copy)]
enum TabCategory {
    Params,
    Authorization,
    Headers,
    Body,
    Settings,
}

enum SettingOptions {}

struct ParamState {}

#[derive(Clone, Copy)]
enum ResponseCategory {
    Body,
    Cookies,
    Headers,
}
