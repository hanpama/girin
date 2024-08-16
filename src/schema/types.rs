use std::collections::HashMap;

#[derive(Debug)]
pub struct Position {
    // pub directory: Vec<String>,
    // pub module: String,
    /// One-based line number
    pub line: usize,

    /// One-based column number
    pub column: usize,
}

#[derive(Debug, Clone)]
pub enum TypeExpression {
    NamedType(String),
    ListType(Box<TypeExpression>),
    NonNullType(Box<TypeExpression>),
}

#[derive(Debug)]
pub enum Value {
    Int(i32),
    Float(f64),
    String(String),
    Boolean(bool),
    Enum(String),
    Null,
    List(Vec<Value>),
    Object(HashMap<String, Value>),
}
