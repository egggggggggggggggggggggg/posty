use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, StatefulWidget, Widget},
};

use crate::{
    input_field::InputBox,
    text_editor::{TextEditor, TextEditorState},
};

#[derive(Default)]
enum Component {
    Endpoint,
    Headers,
    #[default]
    Parameters,
    Body,
    Auth,
}

pub struct Editor {
    selected_component: Component,
    endpoint_input: InputBox,
    body: TextEditorState,
}
impl Widget for &mut Editor {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        match self.selected_component {
            Component::Body => {
                TextEditor::new()
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Cyan))
                            .border_type(BorderType::Rounded)
                            .title("Editor"),
                    )
                    .cursor_style(
                        Style::default()
                            .bg(Color::Cyan)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    )
                    .line_number_style(Style::default().fg(Color::DarkGray))
                    .current_line_style(Style::default().bg(Color::Rgb(30, 30, 50)))
                    .render(area, buf, &mut self.body);
            }
            _ => {}
        }
    }
}
