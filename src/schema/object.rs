use super::{Field, Module, Position, SourceConfig};

#[derive(Debug)]
pub struct ObjectDefinition {
    pub fields: Vec<Field>,
    pub name: String,
    pub description: Option<String>,
    pub interfaces: Vec<String>,
    pub position: Position,
}

impl ObjectDefinition {
    pub fn iter_interfaces(&self) -> impl Iterator<Item = &str> {
        self.interfaces.iter().map(|s| s.as_str())
    }

    pub fn get_field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|field| field.name == name)
    }

    pub fn iter_fields(&self) -> impl Iterator<Item = &Field> {
        self.fields.iter()
    }

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
    pub fn iter_interfaces(&self) -> impl Iterator<Item = &str> {
        self.interfaces.iter().map(|s| s.as_str())
    }

    pub fn get_field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|field| field.name == name)
    }

    pub fn iter_fields(&self) -> impl Iterator<Item = &Field> {
        self.fields.iter()
    }

    pub fn collect_source_configs(&self) -> Vec<SourceConfig> {
        self.fields
            .iter()
            .flat_map(|field| field.collect_source_configs())
            .collect()
    }
}
