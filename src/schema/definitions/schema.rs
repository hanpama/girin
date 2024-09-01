use super::Position;

#[derive(Debug)]
pub struct SchemaDefinition {
    pub query: Option<String>,
    pub mutation: Option<String>,
    pub subscription: Option<String>,
    pub position: Position,
}
