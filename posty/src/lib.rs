use std::{collections::HashMap, fs, path::Path};

use crate::save::Node;

pub mod requests;
pub mod save;
pub mod sharing;

pub fn load_projects(p: impl AsRef<Path>) -> std::io::Result<Node> {
    let contents = fs::read_to_string(p)?;
    let deserialized: Node = serde_json::from_str(&contents)?;
    Ok(deserialized)
}
