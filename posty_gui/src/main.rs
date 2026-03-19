use iced::{
    Element, Length,
    widget::{button, column, container, row, scrollable, text},
};

pub fn main() -> iced::Result {
    iced::run(FolderViewer::update, FolderViewer::view)
}

#[derive(Debug, Clone)]
pub enum Message {
    FolderClicked(String),
    FileClicked(String),
}

#[derive(Default)]
struct FolderViewer {
    current_path: String,
}

impl FolderViewer {
    pub fn view(&self) -> Element<'_, Message> {
        // Fake folder + file data
        let folders = vec!["Documents", "Downloads", "Pictures"];
        let files = vec!["file1.txt", "image.png", "notes.md"];

        let sidebar = column(
            folders
                .into_iter()
                .map(|folder| {
                    button(text(format!("📁 {}", folder)))
                        .on_press(Message::FolderClicked(folder.to_string()))
                        .into()
                })
                .collect::<Vec<_>>(),
        )
        .spacing(5)
        .width(Length::FillPortion(1));
        let file_list = column(
            files
                .into_iter()
                .map(|file| {
                    button(text(format!("📄 {}", file)))
                        .on_press(Message::FileClicked(file.to_string()))
                        .into()
                })
                .collect::<Vec<_>>(),
        )
        .spacing(5);

        let content = column![
            text(format!("Current Path: {}", self.current_path)).size(20),
            scrollable(file_list)
        ]
        .spacing(10)
        .width(Length::FillPortion(7));

        row![
            container(sidebar).width(Length::FillPortion(1)),
            container(content).width(Length::FillPortion(7)),
        ]
        .spacing(20)
        .padding(10)
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::FolderClicked(folder) => {
                self.current_path = format!("/{}", folder);
            }
            Message::FileClicked(file) => {
                println!("Clicked file: {}", file);
            }
        }
    }
}
