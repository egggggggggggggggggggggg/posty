pub mod auth;
pub mod headers;
pub mod params;
pub mod project_bar;
pub mod settings;
pub mod tab_view;
pub mod url_bar;
#[derive(Default, Clone, Copy)]
pub enum WidgetType {
    #[default]
    Empty,
    Input,
    Folder,
    Tabs,
    InputBox,
    Dropdown,
}

pub enum ComponentType {
    Tabview,
    UrlBar,
    Settings,
    Params,
    Headers,
    Auth,
    WidgetType(WidgetType),
}
