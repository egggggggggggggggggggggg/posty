use crossterm::event::{KeyEvent, MouseEvent};

pub trait Actionable {
    fn key_event(&mut self, key: KeyEvent);
    ///Optional implementation, I don't think we need this as the App is responsible
    fn mouse_event(&mut self, mouse: MouseEvent) {}
}
