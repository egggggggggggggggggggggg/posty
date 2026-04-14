use crossterm::event::{KeyEvent, MouseEvent};
use posty::AppEvent;

pub trait Actionable {
    fn key_event(&mut self, key: KeyEvent) -> Option<AppEvent>;
    ///Optional implementation, I don't think we need this as the App is responsible
    fn mouse_event(&mut self, mouse: MouseEvent) {}
}
