use crate::utils::text;

pub fn runtime_spec_type(def_name: &str) -> String {
    format!("{}", escape(def_name))
}

pub fn module_name(module_name: &str) -> String {
    format!("{}", escape(module_name))
}

pub fn module_path(breadcrumbs: &[&str]) -> String {
    breadcrumbs
        .iter()
        .map(|name| module_name(name))
        .collect::<Vec<String>>()
        .join(".")
}

pub fn impl_type(def_name: &str) -> String {
    format!("{}", escape(def_name))
}
pub fn build_config_field(def_name: &str) -> String {
    format!("{}", escape(def_name))
}

pub fn source(def_name: &str) -> String {
    format!("{}", escape(def_name))
}

pub fn source_reference(def_name: &str) -> String {
    // 이거 각 장소로
    format!("source_spec.{}", source(def_name))
}

pub fn type_instance(def_name: &str) -> String {
    format!("{}Type", escape(def_name))
}

pub fn field_name(in_schema_name: &str) -> String {
    escape(&text::to_snake_case(in_schema_name))
}

fn escape(name: &str) -> String {
    text::escape(name, &KEYWORDS)
}

const KEYWORDS: [&str; 40] = [
    "False",
    "def",
    "if",
    "raise",
    "None",
    "del",
    "import",
    "return",
    "True",
    "elif",
    "in",
    "try",
    "and",
    "else",
    "is",
    "while",
    "as",
    "except",
    "lambda",
    "with",
    "assert",
    "finally",
    "nonlocal",
    "yield",
    "break",
    "for",
    "not",
    "class",
    "from",
    "or",
    "continue",
    "global",
    "pass", // reserved
    "str",
    "int",
    "float",
    "bool",
    "graphql",
    "typing",     // import in scope
    "is_type_of", // type resolver
];
