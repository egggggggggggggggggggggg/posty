use crate::key_actions::KeyActions;

pub mod dropdown;
pub mod folder;
pub mod input_box;
pub mod tab;
enum WidgetType {
    Empty,
    Input,
    Folder,
    Tabs,
}
///Trait for Widgets that can cycle between a selection of items via arrow keys. ex: a dropdown
///menu.
pub trait Navigable {
    fn navigate_next(&mut self);
    fn navigate_prev(&mut self);
}
///Any widget that can take Focus, meaning route any KeyActions into itself to be translated. Some
///actions cannot be routed such as Escape.
pub trait Focusable {
    fn on_action(&mut self, key_actions: KeyActions);
}
