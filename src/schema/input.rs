use super::{InputValue, Position, SourceConfig};

#[derive(Debug)]
pub struct Input {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<InputValue>,
    pub position: Position,
}

impl Input {
    pub fn collect_source_configs(&self) -> Vec<SourceConfig> {
        self.fields
            .iter()
            .map(|field| SourceConfig {
                name: field.name.clone(),
                type_: field.field_type.clone(),
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct InputExtension {
    pub name: String,
    pub fields: Vec<InputValue>,
    pub position: Position,
}
