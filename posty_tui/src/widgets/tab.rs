use std::rc::Rc;

use posty::save::ApiRequest;

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

trait WidgetCache {}
#[derive(Default)]
pub struct TabState {
    selected_method: Method,
    selected_category: Category,
    ///Should the widgets be cached, aka don't reconstruct and reuse old saved widgets.
    focused_section: TabSection,
    cache: bool,
    widget_cache: Option<()>,
    pub api_request: ApiRequest,
}
impl TabState {
    pub fn extract(&mut self) -> &ApiRequest {
        &self.api_request
    }
}

impl TabState {}
pub struct Tab {
    state: TabState,
}
impl Tab {}
