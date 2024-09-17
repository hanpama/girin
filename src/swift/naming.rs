use crate::utils::text;

pub fn runtime_spec(def_name: &str) -> String {
    format!("{}", escape(def_name))
}

pub fn source_spec(def_name: &str) -> String {
    format!("{}", escape(def_name))
}

pub fn field_name(field_name: &str) -> String {
    format!("{}", escape(field_name))
}

pub fn enum_value_name(field_name: &str) -> String {
    format!("{}", escape(field_name))
}

pub fn type_instance(def_name: &str) -> String {
    format!("{}Definition", def_name)
}

pub fn source(def_name: &str) -> String {
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

fn escape(name: &str) -> String {
    text::escape(name, &KEYWORDS)
}

const KEYWORDS: [&str; 63] = [
    "Module",
    "GraphQL",
    "Any",
    "Self",
    "as",
    "associatedtype",
    "await",
    "borrowing",
    "break",
    "case",
    "catch",
    "catch",
    "class",
    "consuming",
    "continue",
    "default",
    "defer",
    "deinit",
    "do",
    "else",
    "enum",
    "extension",
    "fallthrough",
    "false",
    "fileprivate",
    "for",
    "func",
    "guard",
    "if",
    "import",
    "in",
    "init",
    "inout",
    "internal",
    "is",
    "let",
    "nil",
    "nonisolated",
    "open",
    "operator",
    "precedencegroup",
    "private",
    "protocol",
    "public",
    "repeat",
    "rethrows",
    "rethrows",
    "return",
    "self",
    "static",
    "struct",
    "subscript",
    "super",
    "switch",
    "throw",
    "throw",
    "throws",
    "true",
    "try",
    "typealias",
    "var",
    "where",
    "while",
];
