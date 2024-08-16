use super::{Module, Position};

#[derive(Debug)]
pub struct EnumDefinition {
    pub name: String,
    pub description: Option<String>,
    pub values: Vec<EnumValue>,
    pub module: Module,
    pub position: Position,
}

#[derive(Debug)]
pub struct EnumValue {
    pub name: String,
    pub description: Option<String>,
    pub deprecation_reason: Option<String>,
    pub position: Position,
}

impl EnumDefinition {
    pub fn iter_values(&self) -> impl Iterator<Item = &EnumValue> {
        self.values.iter()
    }
}

#[derive(Debug)]
pub struct EnumExtension {
    pub name: String,
    pub values: Vec<EnumValue>,
    pub module: Module,
    pub position: Position,
}

impl EnumExtension {
    pub fn iter_values(&self) -> impl Iterator<Item = &EnumValue> {
        self.values.iter()
    }
}
