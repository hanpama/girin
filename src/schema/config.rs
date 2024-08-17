use super::{Field, TypeExpression};

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
