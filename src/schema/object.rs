use super::{Field, Position, SourceConfig};

#[derive(Debug)]
pub struct Object {
    pub fields: Vec<Field>,
    pub name: String,
    pub description: Option<String>,
    pub interfaces: Vec<String>,
    pub position: Position,
}

impl Object {
    pub fn collect_source_configs(&self) -> Vec<SourceConfig> {
        self.fields
            .iter()
            .flat_map(|field| field.collect_source_configs())
            .collect()
    }
}

#[derive(Debug)]
pub struct ObjectExtension {
    pub name: String,
    pub interfaces: Vec<String>,
    pub fields: Vec<Field>,
    pub position: Position,
}

impl ObjectExtension {
    pub fn collect_source_configs(&self) -> Vec<SourceConfig> {
        self.fields
            .iter()
            .flat_map(|field| field.collect_source_configs())
            .collect()
    }
}
