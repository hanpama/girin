use super::{InputValue, Position, TypeExpression};

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub description: Option<String>,
    pub deprecation_reason: Option<String>,
    pub args: Vec<InputValue>,
    pub field_type: TypeExpression,
    pub resolve_config: Option<FieldResolveConfig>,
    pub source_configs: Vec<FieldSourceConfig>,
    // pub type_name: String,
    pub position: Position,
}

pub enum FieldResolverKind {
    Sync,
    Async,
}

impl Field {
    pub fn get_source_configs(&self) -> &Vec<FieldSourceConfig> {
        &self.source_configs
    }

    pub fn has_resolve_config(&self) -> bool {
        self.resolve_config.is_some()
    }

    pub fn get_resolve_config(&self) -> Option<&FieldResolveConfig> {
        self.resolve_config.as_ref()
    }
}

#[derive(Debug)]
pub struct FieldResolveConfig {
    pub sync: bool,
}

#[derive(Debug, Clone)]
pub struct FieldSourceConfig {
    pub name: String,
    pub type_: TypeExpression,
}
