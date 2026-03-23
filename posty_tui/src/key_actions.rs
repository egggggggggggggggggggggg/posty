use crossterm::event::KeyCode;

use crate::widgets::WidgetType;

#[derive(Default)]
pub enum CursorMode {
    #[default]
    Insert,
    Normal,
}
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

pub enum KeyActions {
    ///Escape can be used for anything as the widget has to first handle everything that happens
    ///when it loses focus. LoseFocus must always be emitted after Escape is sent. This exists just
    ///so widgets can safely define behavior that should happen on lost focus.
    Escape,
    ///Focuses on an arbtitrary widget, haven't decided how we wanna abstract the widgets to make
    ///it easier to select. When a widget has focus the characters must be sent immmediately to it
    ///unless it falls under one of the specific keybinds that should not be sent. eg: chars can be
    ///sent freely but Esc, etc cannot.
    Focus(WidgetType),
    ///Wrapper around a KeyCode when there is focus on a widget, any keycode is transferred over to
    ///said focused widget to handle.
    Input(KeyCode),
    ///For when the arrow keys or any specified key mapped to a direction is pressed, same idea as
    ///above although this is more specific. Only emitted when the
    MoveDirection(Direction),
    ///Action that is sent back to the main app telling it to hide the cursor/aka lose focus.
    LoseFocus,
    ///Mode change, unsure whether this should be a part of LoseFocus. This is just to ensure I
    ///remember to implement some way of changing the cursor mode.
    CursorModeChange(CursorMode),
    ///Command mode
    Command,
    ///Both kill and bring toggle the visible popup bool so the renderer can determine what to do.
    ///Brings up the current visible popup for viewing.
    BringPopup,
    ///Kills the current visible popup.
    KillPopup,
    ///Enters the current input or selects the current item. Depends on the widgets implementation.
    ///Should just be for entering input though.
    Enter,
    ///The raw chars are sent for the widget to intrepet.
    Char(char),
}
