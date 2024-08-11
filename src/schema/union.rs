use super::Position;

#[derive(Debug)]
pub struct Union {
    pub name: String,
    pub description: Option<String>,
    pub types: Vec<String>,
    pub position: Position,
}

#[derive(Debug)]
pub struct UnionExtension {
    pub name: String,
    pub types: Vec<String>,
    pub position: Position,
}
