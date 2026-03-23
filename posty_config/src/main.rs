///Theres probably a better way of doing this instead of importing the whole crate.  
use ratatui::crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}
#[derive(Serialize, Deserialize)]
pub enum KeyAction {}
struct Config {
    tui_keymaps: HashMap<KeyCode, KeyAction>,
}

impl Config {
    fn load() {}
}
