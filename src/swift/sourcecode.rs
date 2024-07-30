use std::collections::HashMap;

pub struct SourceCode {
    prelude: String,
    indent_unit: String,
    indent_level: i32,
    modules: HashMap<String, bool>,
    body: String,
    list_index: i32,
}
