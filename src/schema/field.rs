use super::{InputValue, Position, TypeExpression};

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub description: Option<String>,
    pub deprecation_reason: Option<String>,
    pub args: Vec<InputValue>,
    pub field_type: TypeExpression,
    pub resolve: Option<ResolveConfig>,
    pub source_configs: Vec<SourceConfig>,
    pub type_name: String,
    pub position: Position,
}

impl Field {
    pub fn collect_source_configs(&self) -> Vec<SourceConfig> {
        if !self.source_configs.is_empty() {
            return self.source_configs.clone();
        } else if self.resolve.is_some() {
            return Vec::new();
        } else if !self.args.is_empty() {
            return Vec::new();
        } else {
            vec![SourceConfig {
                name: self.name.clone(),
                type_: self.field_type.clone(),
            }]
        }
    }
}

#[derive(Debug)]
pub struct ResolveConfig {
    pub sync: bool,
}

pub struct Resolve<'a> {
    pub sync: bool,
    pub field: &'a Field,
}

impl Resolve<'_> {
    pub fn new<'a>(sync: bool, field: &'a Field) -> Resolve<'a> {
        Resolve { sync, field }
    }
}

#[derive(Debug, Clone)]
pub struct SourceConfig {
    pub name: String,
    pub type_: TypeExpression,
}
