use std::collections::HashMap;

use super::{Module, Position};

#[derive(Debug)]
pub struct ScalarDefinition {
    pub name: String,
    pub description: Option<String>,
    pub position: Position,
    pub type_aliases: HashMap<String, String>,
}
