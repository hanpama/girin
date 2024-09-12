use super::naming;
use crate::schema::TypeExpression;

pub fn format_type_expression(expr: &TypeExpression) -> String {
    match expr {
        TypeExpression::NonNullType(inner) => match inner.as_ref() {
            TypeExpression::NamedType(ref name) => format_named_type(name),
            TypeExpression::ListType(inner) => {
                format!("[{}]", format_type_expression(inner.as_ref()))
            }
            _ => unreachable!(),
        },
        TypeExpression::NamedType(name) => {
            format!("{}?", format_named_type(name))
        }
        TypeExpression::ListType(inner) => {
            format!("[{}]?", format_type_expression(inner.as_ref()))
        }
    }
}

fn format_named_type(name: &str) -> String {
    match name {
        "String" => return "String".to_owned(),
        "Int" => return "Int".to_owned(),
        "Float" => return "Float".to_owned(),
        "Boolean" => return "Bool".to_owned(),
        "ID" => return "Any".to_owned(),
        _ => format_named_source(name),
    }
}

pub fn format_named_source(def_name: &str) -> String {
    format!("SourceSpec.{}", naming::source_spec(def_name))
}
