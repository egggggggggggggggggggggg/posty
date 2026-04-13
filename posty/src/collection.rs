use crate::{RequestData, ResponseData};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Node {
    pub name: String,
    pub kind: NodeKind,
}
impl Node {
    pub fn expand(&mut self) {
        if let NodeKind::Folder(folder) = &mut self.kind {
            folder.expanded = true;
        }
    }
    pub fn add_child(&mut self, child: Node) {
        if let NodeKind::Folder(folder) = &mut self.kind {
            folder.children.push(child);
        }
    }
    ///Simple linear search. If we need better performance(when data gets larger), we can rewrite
    ///this using `HashMap`
    pub fn find_file(&self, path: &[String]) -> Option<FileData> {
        if path.is_empty() {
            return None;
        }
        if self.name != path[0] {
            return None;
        }
        match &self.kind {
            NodeKind::File(f) => {
                if path.len() == 1 {
                    Some(f.clone())
                } else {
                    None
                }
            }
            NodeKind::Folder(folder) => {
                if path.len() == 1 {
                    return None;
                }
                folder
                    .children
                    .iter()
                    .find_map(|child| child.find_file(&path[1..]))
            }
        }
    }
    pub fn remove(&mut self, path: &[String]) -> Option<Node> {
        if path.is_empty() {
            return None;
        }

        // The current node must match the first path segment
        if self.name != path[0] {
            return None;
        }

        match &mut self.kind {
            NodeKind::File(_) => {
                // Can't remove deeper from a file
                None
            }
            NodeKind::Folder(folder) => {
                // If this is the parent of the target
                if path.len() == 2 {
                    let target_name = &path[1];

                    if let Some(pos) = folder
                        .children
                        .iter()
                        .position(|child| &child.name == target_name)
                    {
                        return Some(folder.children.remove(pos));
                    }

                    None
                } else if path.len() > 2 {
                    // Recurse deeper
                    folder
                        .children
                        .iter_mut()
                        .find_map(|child| child.remove(&path[1..]))
                } else {
                    //Can't remove self
                    None
                }
            }
        }
    }
    pub fn rename(&mut self, path: &[String], new_name: &str) -> Option<()> {
        if path.is_empty() || self.name != path[0] {
            return None;
        }
        if path.len() == 1 {
            self.name = new_name.to_string();
            return Some(());
        } else {
            match &mut self.kind {
                NodeKind::File(_) => None,
                NodeKind::Folder(folder) => folder
                    .children
                    .iter_mut()
                    .find_map(|child| child.rename(&path[1..], new_name)),
            }
        }
    }
    #[inline(always)]
    pub fn is_folder(&self) -> bool {
        if let NodeKind::Folder(_) = self.kind {
            true
        } else {
            false
        }
    }
    #[inline(always)]
    pub fn is_file(&self) -> bool {
        if let NodeKind::File(_) = self.kind {
            true
        } else {
            false
        }
    }
}
#[derive(Serialize, Deserialize)]
pub enum NodeKind {
    File(FileData),
    Folder(FolderData),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FileData {
    pub response: Option<ResponseData<'static>>,
    pub request: RequestData,
}

#[derive(Serialize, Deserialize)]
pub struct FolderData {
    pub children: Vec<Node>,
    pub expanded: bool,
}
