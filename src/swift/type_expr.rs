use super::naming;
use crate::schema::TypeExpression;

pub fn format_type_expression(source_ns: Option<&str>, expr: &TypeExpression) -> String {
    match expr {
        TypeExpression::NonNullType(inner) => match inner.as_ref() {
            TypeExpression::NamedType(ref name) => format_named_type(source_ns, name),
            TypeExpression::ListType(inner) => {
                format!("[{}]", format_type_expression(source_ns, inner.as_ref()))
            }
            _ => unreachable!(),
        },
        TypeExpression::NamedType(name) => {
            format!("{}?", format_named_type(source_ns, name))
        }
        TypeExpression::ListType(inner) => {
            format!("[{}]?", format_type_expression(source_ns, inner.as_ref()))
        }
    }
}

pub fn format_named_type(source_ns: Option<&str>, name: &str) -> String {
    match name {
        "String" => return "String".to_owned(),
        "Int" => return "Int".to_owned(),
        "Float" => return "Float".to_owned(),
        "Boolean" => return "Bool".to_owned(),
        "ID" => return "String".to_owned(),
        _ => {
            if let Some(ns) = source_ns {
                format!("{}.{}", ns, naming::source(name))
            } else {
                naming::source(name)
            }
        }
    }
}
