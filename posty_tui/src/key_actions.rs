use crossterm::event::KeyCode;

pub enum KeyActions {
    ///Escapes out of a current widget, eg an InputField
    Escape,
    ///Focuses on an arbtitrary widget, haven't decided how we wanna abstract the widgets to make
    ///it easier to select.
    Focus(),
    ///Wrapper around a KeyCode when there is focus on a widget, any keycode is transferred over to
    ///said focused widget to handle.
    Input(KeyCode),
    ///For when the arrow keys or any specified key mapped to a direction is pressed, same idea as
    ///above although this is more specific. Only emitted when the
    MoveDirection(),
}
