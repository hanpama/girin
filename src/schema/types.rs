use std::collections::HashMap;

#[derive(Debug)]
pub struct Position {
    // pub file: String,
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

#[derive(Clone)]
pub struct Loc<'a> {
    pub directory: Vec<&'a str>,
    pub module: &'a str,
    pub definition: &'a str,
}

impl<'a> Loc<'a> {
    pub fn new(module: &'a str, definition: &'a str) -> Self {
        Self {
            directory: Vec::new(),
            module,
            definition,
        }
    }

    fn prepend(mut self, name: &'a str) -> Loc<'a> {
        self.directory.insert(0, name);
        self
    }
}
