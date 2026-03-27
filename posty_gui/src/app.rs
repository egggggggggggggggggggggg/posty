use iced::Element;

pub enum Message {
    Clicked(String),
}

pub struct App {}
impl App {
    pub fn update(&mut self, message: Message) {}
}
