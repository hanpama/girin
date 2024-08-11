use super::TypeExpression;

#[derive(Debug)]
pub struct ResolverConfig {
    pub sync: bool,
}

#[derive(Debug, Clone)]
pub struct SourceConfig {
    pub name: String,
    pub type_: TypeExpression,
}
