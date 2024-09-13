use crate::utils::text;

pub fn runtime_spec(def_name: &str) -> String {
    format!("{}", escape(def_name))
}

pub fn source_spec<S: Into<String>>(def_name: S) -> String {
    format!("{}", def_name.into())
}

pub fn field_name<S: Into<String>>(field_name: S) -> String {
    format!("{}", field_name.into())
}

pub fn type_instance(def_name: &str) -> String {
    format!("{}Definition", def_name)
}
pub fn type_defining_function(def_name: &str) -> String {
    format!("define{}", def_name)
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

const KEYWORDS: [&str; 2] = ["Module", "GraphQL"];
