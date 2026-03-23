use posty::save::ApiRequest;
use ratatui::{
    layout::Rect,
    widgets::{StatefulWidget, Table, TableState},
};

use crate::{
    key_actions::KeyActions,
    widgets::{
        Actionable, WidgetType,
        dropdown::Dropdown,
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
impl Method {
    fn next(self) -> Self {
        match self {
            Method::Get => Method::Post,
            Method::Post => Method::Set,
            Method::Set => Method::Patch,
            Method::Patch => Method::Put,
            Method::Put => Method::Delete,
            Method::Delete => Method::Get,
        }
    }

    fn prev(self) -> Self {
        match self {
            Method::Get => Method::Delete,
            Method::Post => Method::Get,
            Method::Set => Method::Post,
            Method::Patch => Method::Set,
            Method::Put => Method::Patch,
            Method::Delete => Method::Put,
        }
    }
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
#[derive(Default)]
pub struct TabState {
    selected_method: Method,
    selected_category: Category,
    ///Should the widgets be cached, aka don't reconstruct and reuse old saved widgets.
    focused_section: TabSection,
    pub api_request: ApiRequest,
    ///There are levels to focus. Widget focus and overall app focus. widget focus is local hte
    ///defined widget while App focus is for all of the present widgets.
    focused_widget: WidgetType,
    input_box: InputBoxState,
    method_dropdown: Dropdown,
    param_table: TableState,
    cursor_pos: (usize, usize),
    ///This is allocated on creation of tab that alllows for dynamic widgets that change,
    arbitrary_area: Rect,
    response_section: ResponseSection,
}
#[derive(Default)]
pub struct ResponseSection {}
impl ResponseSection {}
impl TabState {
    pub fn extract(&mut self) -> &ApiRequest {
        &self.api_request
    }
    ///Sends back an action that the poller must act on.
    pub fn key_action(&mut self, action: KeyActions) -> Option<KeyActions> {
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
    ///This does not handle losing focus. That is handled in the key_action function.
    pub fn change_focus(&mut self, new_wiget: WidgetType) {}
    ///The focus of a widget changedd by the postion of the cursor placement.
    pub fn cursor_focus(&mut self, x: usize, y: usize) {}
}
pub struct Tab;
impl StatefulWidget for Tab {
    type State = TabState;
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {}
}
