use crate::key_actions::KeyActions;

pub mod commands;
pub mod dropdown;
pub mod folder;
pub mod input_box;
pub mod response_section;
pub mod tab;

#[derive(Default)]
pub enum WidgetType {
    #[default]
    Empty,
    Input,
    Folder,
    Tabs,
    InputBox,
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

///Trait for all the scattered key_actions found in the widget area.
///This should be implemented for the state of the widget if the widget implements StatefulWidget.
///It is also possible for this function to do nothing. This allows for implementing it in places
///where we normally wouldn't want anything to happen. (Flexibility mainly is the reason for this).
pub trait Actionable {
    fn key_actions(&mut self, key_actions: KeyActions) -> Option<KeyActions>;
}
