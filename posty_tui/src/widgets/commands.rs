use crate::{
    key_actions::KeyActions,
    widgets::{
        Actionable,
        input_box::{InputBox, InputBoxState},
    },
};
use ratatui::widgets::StatefulWidget;

#[derive(Default)]
pub struct CommandState {
    input_box: InputBoxState,
}
impl CommandState {
    fn new() {}
    fn execute(&mut self) -> Option<KeyActions> {
        let content = self.input_box.content();
        let (cmd, args) = parse_args_and_cmd(&content);
        match cmd {
            Commands::New => {
                //creates a new proejct with a given name.
            }
            Commands::Open => {}
            Commands::Delete => {}
            _ => {
                println!("Unrecognized questions.")
            }
        }
        self.input_box.clear();
        None
    }
}
#[inline(always)]
//Should change this to Vec<char> or smth of the sort.
fn parse_args_and_cmd(buf: &str) -> (Commands, Vec<Args>) {
    //placeholder for now.
    (Commands::New, Vec::new())
}
///thin wrappe around input box
#[derive(Default)]
pub struct CommandBox {
    input_box: InputBox,
}
impl StatefulWidget for CommandBox {
    type State = CommandState;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        self.input_box.render(area, buf, &mut state.input_box);
    }
}
impl Actionable for CommandState {
    fn key_actions(&mut self, key_actions: KeyActions) -> Option<KeyActions> {
        match key_actions {
            KeyActions::Enter => {
                self.execute();
            }
            _ => {
                self.input_box.key_actions(key_actions);
            }
        }
        None
    }
}
enum Commands {
    New,
    Open,
    Delete,
}
enum Args {
    FilePath(String),
    Name(String),
}
