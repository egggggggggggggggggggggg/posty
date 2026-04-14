use crossterm::event::KeyCode;
use posty::collection::{Node, NodeKind, NodeType};
use ratatui::widgets::Widget;

use crate::{AppEvent, action::Actionable};

pub struct DisplayNode {
    path: Vec<String>,
    expanded: bool,
    is_folder: bool,
}
pub struct CollectionPane {
    items: Node,
    focused_node: usize,
    visible_nodes: Vec<DisplayNode>,
    ///If the user is currently naming a new Node.
    editing: bool,
    node_editor: NodeEditor,
}
impl CollectionPane {
    ///Node is read from a file and passed in here where it displays stuff.
    pub fn new(items: Node) -> Self {
        let mut pane = Self {
            items,
            focused_node: 0,
            visible_nodes: Vec::new(),
            editing: false,
            node_editor: NodeEditor::default(),
        };
        pane.rebuild_visible_nodes();
        pane
    }
    fn collect_visible(node: &Node, path: &mut Vec<String>, out: &mut Vec<DisplayNode>) {
        match &node.kind {
            NodeKind::File(f) => {}
            NodeKind::Folder(folder) => {
                for child in &folder.children {
                    let mut child_path = path.clone();
                    child_path.push(child.name.to_string());
                    let is_folder = child.is_folder();
                    let expanded = out
                        .iter()
                        .find(|d| d.path == child_path)
                        .map(|d| d.expanded)
                        .unwrap_or(false);
                    out.push(DisplayNode {
                        path: child_path.clone(),
                        expanded,
                        is_folder,
                    });
                    if is_folder && expanded {
                        Self::collect_visible(child, &mut child_path, out);
                    }
                }
            }
        }
    }
    fn rebuild_visible_nodes(&mut self) {
        let expanded_paths: Vec<Vec<String>> = self
            .visible_nodes
            .iter()
            .filter(|d| d.expanded)
            .map(|d| d.path.clone())
            .collect();
        let mut new_nodes: Vec<DisplayNode> = Vec::new();
        Self::collect_visible(&self.items, &mut Vec::new(), &mut new_nodes);
        for node in &mut new_nodes {
            if expanded_paths.contains(&node.path) {
                node.expanded = true;
            }
        }
        self.visible_nodes = new_nodes;
        if !self.visible_nodes.is_empty() {
            self.focused_node = self.focused_node.midpoint(self.visible_nodes.len() - 1);
        } else {
            self.focused_node = 0;
        }
    }
    fn act_at_focused(&mut self) -> Option<AppEvent> {
        if self.visible_nodes.is_empty() {
            return None;
        }
        let focused = &self.visible_nodes[self.focused_node];
        if focused.is_folder {
            self.visible_nodes[self.focused_node].expanded = !focused.expanded;
            self.rebuild_visible_nodes();
            None
        } else {
            let fd = self.items.find_file(&focused.path).unwrap();
            Some(AppEvent::ChangeDisplay(fd))
        }
    }
    fn remove_at_focused(&mut self) {
        if self.visible_nodes.is_empty() {
            return;
        }
        let path = self.visible_nodes[self.focused_node].path.clone();
        self.items.remove(&path);
        self.rebuild_visible_nodes();

        if !self.visible_nodes.is_empty() && self.focused_node >= self.visible_nodes.len() {
            self.focused_node = self.visible_nodes.len() - 1;
        }
    }
    fn down(&mut self) {
        if self.visible_nodes.len() < 2 {
            self.focused_node = 0;
            return;
        }
        self.focused_node = (self.focused_node + 1).min(self.visible_nodes.len() - 1);
    }
    fn up(&mut self) {
        if self.visible_nodes.len() < 2 {
            self.focused_node = 0;
            return;
        }
        self.focused_node = self.focused_node.saturating_sub(1);
    }
}

impl Actionable for CollectionPane {
    fn key_event(&mut self, key: crossterm::event::KeyEvent) -> Option<AppEvent> {
        if self.editing {
            return self.node_editor.key_event(key);
        }
        match key.code {
            KeyCode::Enter => {
                return self.act_at_focused();
            }
            KeyCode::Down => {
                self.down();
            }
            KeyCode::Up => {
                self.up();
            }
            KeyCode::Backspace => {
                self.remove_at_focused();
            }
            KeyCode::Char('a') => {}
            KeyCode::Char('r') => {}
            _ => {}
        }
        None
    }
}

///This might be redundant but it's easier to seperate logic and avoid clotting up the keybind
///actions for CollectionPane. Makes it easier to reason about the code. Add a way of cahnging the
///NodeType later. Either thru a visual element or a keybind
#[derive(Default)]
pub struct NodeEditor {
    name_buffer: String,
    node_type: NodeType,
}
impl Actionable for NodeEditor {
    fn key_event(&mut self, key: crossterm::event::KeyEvent) -> Option<AppEvent> {
        match key.code {
            KeyCode::Char(ch) => {
                self.name_buffer.push(ch);
            }
            KeyCode::Backspace => {
                if self.name_buffer.is_empty() {
                    return None;
                }
                self.name_buffer.pop();
            }
            KeyCode::Enter => match self.node_type {
                NodeType::File => return Some(todo!("")),
                _ => return None,
            },
            _ => {}
        }
        None
    }
}
impl Widget for &NodeEditor {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
    }
}
