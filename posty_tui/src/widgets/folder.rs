use std::path::Path;

use posty::{
    load_projects,
    save::{ApiRequest, Node},
};
use ratatui::{
    style::{Modifier, Style},
    text::Line,
    widgets::{List, ListItem, StatefulWidget, Widget},
};

#[derive(Clone)]
pub struct VisibleNode {
    pub path: Vec<String>,
    pub depth: usize,
    pub is_dir: bool,
    pub expanded: bool,
}
impl VisibleNode {
    pub fn name(&self) -> &str {
        self.path.last().map(String::as_str).unwrap_or("")
    }
}
#[derive(Default)]
pub struct Explorer {
    //this is really dumb
    pub projects: Option<Node>,
    pub visible: Vec<VisibleNode>,
    pub cursor: usize,
}
impl Explorer {
    pub fn load(&mut self, p: impl AsRef<Path>) {
        let projects = load_projects(p).unwrap();
        self.projects = Some(projects);
        self.build_visible();
    }
    #[inline(always)]
    pub fn cursor_up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }
    #[inline(always)]
    pub fn cursor_down(&mut self) {
        if !self.visible.is_empty() && self.cursor < self.visible.len() - 1 {
            self.cursor += 1;
        }
    }

    pub fn expand_at(&mut self, index: usize) {
        let tokens: Vec<String> = self.visible[index].path.clone();
        find_dir_mut(&mut self.projects.as_mut().unwrap().clone(), &tokens).expand();
        self.build_visible();
    }
    pub fn collapse_at(&mut self, index: usize) {
        let tokens: Vec<String> = self.visible[index].path.clone();
        find_dir_mut(&mut self.projects.as_mut().unwrap().clone(), &tokens).expand();
        self.build_visible();
        if self.cursor >= self.visible.len() {
            self.cursor = self.visible.len().saturating_sub(1);
        }
    }
    pub fn toggle_at_cursor(&mut self) {
        if self.visible.is_empty() {
            return;
        };
        let cursor_node = &mut self.visible[self.cursor];
        cursor_node.expanded = !cursor_node.expanded;
        if cursor_node.expanded {
            self.expand_at(self.cursor);
        } else {
            self.collapse_at(self.cursor);
        }
    }
    pub fn build_visible(&mut self) {
        self.visible.clear();
        let node = self.projects.clone();
        self.walk(&node.unwrap(), 0, &mut Vec::new());
    }
    pub fn walk(&mut self, node: &Node, depth: usize, path: &mut Vec<String>) {
        match node {
            Node::File { name, .. } => {
                path.push(name.clone());
                self.visible.push(VisibleNode {
                    path: path.clone(),
                    depth,
                    is_dir: false,
                    expanded: false,
                });
                path.pop();
            }
            Node::Dir {
                name,
                children,
                expanded,
            } => {
                path.push(name.clone());
                self.visible.push(VisibleNode {
                    path: path.clone(),
                    depth,
                    is_dir: true,
                    expanded: *expanded,
                });
                if *expanded {
                    for child in children {
                        self.walk(child.1, depth + 1, path);
                    }
                }
                path.pop();
            }
        }
    }
    #[inline(always)]
    pub fn cursor_is_dir(&mut self) -> bool {
        if self.visible.is_empty() {
            panic!("The visible section is empty, cursor cannot select anything");
        }
        let target_node = &self.visible[self.cursor];
        target_node.is_dir
    }
    pub fn file_content_at_cursor(&mut self) -> (&String, &ApiRequest) {
        let target_node_path = &self.visible[self.cursor].path;
        let node = find_dir(&self.projects.as_ref().unwrap(), target_node_path);
        if let Node::File { name, request_info } = node {
            (name, request_info)
        } else {
            panic!("Never call this method on a folder member");
        }
    }
    pub fn to_list(&self) -> List {
        let items: Vec<ListItem> = self
            .visible
            .iter()
            .enumerate()
            .map(|(i, node)| {
                let indent = "  ".repeat(node.depth);
                let symbol = if node.is_dir {
                    if node.expanded { " " } else { " " }
                } else {
                    "  "
                };
                let line = Line::from(format!("{}{}{}", indent, symbol, node.name()));
                if i == self.cursor {
                    ListItem::new(line).style(Style::default().add_modifier(Modifier::REVERSED))
                } else {
                    ListItem::new(line)
                }
            })
            .collect();

        List::new(items)
    }
}

#[inline(always)]
fn find_dir_mut<'a>(node: &'a mut Node, path: &[String]) -> &'a mut Node {
    match node {
        Node::Dir {
            name: _,
            children,
            expanded: _,
        } => {
            let child = children.get_mut(&path[0]).unwrap();
            if path.len() == 1 {
                return child;
            }
            let remaining_path = &path[1..];
            find_dir_mut(child, remaining_path)
        }
        Node::File {
            name,
            request_info: _,
        } => {
            if name == &path[0] {
                return node;
            } else {
                panic!("Could not find node.")
            }
        }
    }
}
#[inline(always)]
fn find_dir<'a>(node: &'a Node, path: &[String]) -> &'a Node {
    match node {
        Node::Dir {
            name: _,
            children,
            expanded: _,
        } => {
            let child = children.get(&path[0]).unwrap();
            if path.len() == 1 {
                return child;
            }
            let remaining_path = &path[1..];
            find_dir(child, remaining_path)
        }
        Node::File {
            name,
            request_info: _,
        } => {
            if name == &path[0] {
                return node;
            } else {
                panic!("Could not find node.")
            }
        }
    }
}
