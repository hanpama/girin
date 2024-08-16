use super::{InputValue, Module, Position};

#[derive(Debug)]
pub struct InputDefinition {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<InputValue>,
    pub module: Module,
    pub position: Position,
}

impl InputDefinition {
    pub fn iter_fields(&self) -> impl Iterator<Item = &InputValue> {
        self.fields.iter()
    }
}

#[derive(Debug)]
pub struct InputExtension {
    pub name: String,
    pub fields: Vec<InputValue>,
    pub module: Module,
    pub position: Position,
}

impl InputExtension {
    pub fn iter_fields(&self) -> impl Iterator<Item = &InputValue> {
        self.fields.iter()
    }
}
