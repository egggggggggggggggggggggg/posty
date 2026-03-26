pub mod auth;
pub mod headers;
pub mod params;
pub mod settings;
pub mod tab_view;
pub mod url_bar;
pub enum FocusedCategory {
    Params,
    Headers,
    Auth,
    Settings,
}
