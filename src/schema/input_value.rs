use super::{Position, TypeExpression, Value};

#[derive(Debug)]
pub struct InputValue {
    pub name: String,
    pub description: Option<String>,
    pub deprecation_reason: Option<String>,
    pub field_type: TypeExpression,
    pub default_value: Option<Value>,
    pub position: Position,
}
