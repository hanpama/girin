use super::{Field, Position, SourceConfig};

#[derive(Debug)]
pub struct Interface {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<Field>,
    pub interfaces: Vec<String>,
    pub position: Position,
}

impl Interface {
    pub fn collect_source_configs(&self) -> Vec<SourceConfig> {
        self.fields
            .iter()
            .flat_map(|field| field.collect_source_configs())
            .collect()
    }
}

#[derive(Debug)]
pub struct InterfaceTypeExtension {
    pub name: String,
    pub fields: Vec<Field>,
    pub interfaces: Vec<String>,
    pub position: Position,
}
