use crate::key_actions::KeyActions;

///Rewriting cause a lot of the widgets I wrote didn't require a seperate state struct.
pub mod dropdown;
pub mod i_table;
pub mod input_box;
pub mod input_table;
pub mod tabs;
pub mod toggle;
pub mod tree;
#[derive(Clone, Copy)]
pub enum WidgetType {
    Empty,
    Dropdown,
    InputBox,
    InputTable,
    Toggle,
    Tree,
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
