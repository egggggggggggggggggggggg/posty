use ratatui::widgets::Widget;

use crate::input_field::InputBox;
//Lots of comments here but these are just ideas to flesh out later. Will remove when implemented.
pub enum RequestCategory {}
pub enum CommandType {
    Creation,
    Insertion,
    Cateogry(RequestCategory),
    Deletion,
    Incomplete(String),
    Arg(Argument),
}
pub enum Argument {
    String(String),
}

impl CommandType {
    fn classify() {}
    fn closest_match() {}
}

//Given an arbitrary token(Basically a word seperated by spaces) the auto complete returns the
//closest match. This is not a fuzzy match. Haven't decided edge cases like determining completed
//token or not. Context aware auto complete? Look at previous token and decide if proceede with
//more options.
///Small wrapper around input_box.
//Things the auto complete should support.
//Fill in commands.
//Detecting the end of a command and whether it should continue auto completing or stop to allow
//the user to pass in args.
//Warn the user if they've passed in too many arguments to a command.
//Support undo of an auto completed token.
use std::collections::HashMap;

#[derive(Default)]
pub struct BetterNode {
    pub content: &'static str,
    pub is_terminal: bool,
}

///Replacing this with a constant depth structure instead.
///More like a zig zag where it goes from depth 1 to 2 back to 1, etc.

#[derive(Default)]
pub struct CommandNode {
    pub children: HashMap<String, CommandNode>,
    pub is_terminal: bool,
}
///The node should have a cost to reach their most nested child. This allows us to weigh what
///token should be auto completed. However this does not account for the context in which the next
///token should be used with the previous one. Context aware search is just looking at the previous
///token and checking if the current incomplete token is a possible children of said token.
///Oversimplified but thats the basic idea?  
pub struct TraversalState<'a> {
    pub node: &'a CommandNode,
    pub depth: usize, // how many tokens are fully matched
}
///This only supports validating the node, it should be able to find the next best match.
impl<'a> TraversalState<'a> {
    pub fn new(root: &'a CommandNode) -> Self {
        Self {
            node: root,
            depth: 0,
        }
    }
    pub fn update(&mut self, root: &'a CommandNode, tokens: &[String]) -> Option<&'a CommandNode> {
        // Case 1: tokens got shorter → reset (user deleted)
        if tokens.len() < self.depth {
            *self = Self::new(root);
        }
        // Walk forward only from where we left off
        let mut node = self.node;

        for token in tokens
            .iter()
            .skip(self.depth)
            .take(tokens.len().saturating_sub(1))
        {
            node = node.children.get(token)?;
            self.depth += 1;
        }
        self.node = node;
        Some(node)
    }
}

pub enum TokenKind {
    Literal(String),
    ///Anything that doesn't fall under a literal.
    Argument,
}
impl TokenKind {
    ///Can the token continue turning into a literal. Before searching for a literal match, this
    ///method should be called.
    pub fn viable_literal() {}
    ///Closest literal match.
    pub fn closest_literal() {}
    ///Returns the best match based off of context(previous tokens) and the token that matches the
    ///current prefix.
    pub fn best_literal_match(last_token: TokenKind, prefix: Vec<char>) {}
}

#[derive(Default)]
pub struct CommandPopup {
    inner: InputBox,
    is_space: bool,
    ///Indicator so we know if the current token is an arg. If it is an arg we should not attempt
    ///to provide auto complete.
    is_arg: bool,
    current_token: Vec<char>,
    tokens: Vec<String>,
}

impl CommandPopup {
    pub fn new() -> Self {
        Self::default()
    }
    ///Gets the current token in the input_box stream. This should be rarely used as on insertion
    ///the function checks for if the char is a space. If yes that means the previous token is
    ///done.
    pub fn cursor_token(&self) -> String {
        let left_start = self
            .inner
            .left
            .iter()
            .rposition(|&c| c == ' ')
            .map_or(0, |pos| pos + 1);
        let right_end = self
            .inner
            .right
            .iter()
            .position(|&c| c == ' ')
            .unwrap_or(self.inner.right.len());
        let token: String = self.inner.left[left_start..]
            .iter()
            .chain(self.inner.right[..right_end].iter())
            .collect();
        token
    }
    pub fn insert_char(&mut self, c: char) {
        let is_space = c == ' ';
        if is_space {
            if !self.is_space && !self.current_token.is_empty() {
                self.collect_token();
            }
            self.is_space = true;
        } else {
            if self.is_space {
                self.current_token.clear();
            }
            self.current_token.push(c);
            self.is_space = false;
        }
        self.inner.insert_char(c);
    }
    pub fn collect_token(&mut self) {
        if !self.current_token.is_empty() {
            let token: String = std::mem::take(&mut self.current_token)
                .into_iter()
                .collect();
            self.tokens.push(token);
        }
    }
}

impl Widget for CommandPopup {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
    }
}
