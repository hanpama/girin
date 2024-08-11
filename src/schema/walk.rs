use super::{SchemaElement, Schema};

pub struct Directory<'a> {
    pub schema: &'a Schema,
    pub path: Vec<&'a str>,
}

pub struct File<'a> {
    pub schema: &'a Schema,
    pub path: Vec<&'a str>,
}

impl Schema {}
