use std::collections::BTreeMap;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Position {
    pub file: PathBuf,
    /// One-based line number
    pub line: usize,
    /// One-based column number
    pub column: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file.display(), self.line, self.column)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeExpression {
    NamedType(String),
    ListType(Box<TypeExpression>),
    NonNullType(Box<TypeExpression>),
}

impl TypeExpression {
    pub fn is_nullable(&self) -> bool {
        match self {
            TypeExpression::NonNullType(_) => false,
            _ => true,
        }
    }
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
    Object(BTreeMap<String, Value>),
}
