use super::{Module, Position};

#[derive(Debug)]
pub struct UnionDefinition {
    pub name: String,
    pub description: Option<String>,
    pub types: Vec<String>,
    pub position: Position,
}

impl UnionDefinition {
    pub fn iter_types(&self) -> impl Iterator<Item = &str> {
        self.types.iter().map(|s| s.as_str())
    }
}

#[derive(Debug)]
pub struct UnionExtension {
    pub name: String,
    pub types: Vec<String>,
    pub position: Position,
}

impl UnionExtension {
    pub fn iter_types(&self) -> impl Iterator<Item = &str> {
        self.types.iter().map(|s| s.as_str())
    }
}
