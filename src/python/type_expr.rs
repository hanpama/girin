use crate::{python::naming, schema::TypeExpression};

pub fn format_type_expression(expr: &TypeExpression) -> String {
    match expr {
        TypeExpression::NonNullType(inner) => match inner.as_ref() {
            TypeExpression::NamedType(ref name) => format_named_type(name),
            TypeExpression::ListType(inner) => {
                format!("list[{}]", format_type_expression(inner.as_ref()))
            }
            _ => unreachable!(),
        },
        TypeExpression::NamedType(name) => {
            format!("{} | None", format_named_type(name))
        }
        TypeExpression::ListType(inner) => {
            format!("list[{}] | None", format_type_expression(inner.as_ref()))
        }
    }
}

fn format_named_type(name: &str) -> String {
    match name {
        "String" => return "str".to_owned(),
        "Int" => return "int".to_owned(),
        "Float" => return "float".to_owned(),
        "Boolean" => return "bool".to_owned(),
        "ID" => return "object".to_owned(),
        _ => format_named_source(name),
    }
}

pub fn format_named_source(def_name: &str) -> String {
    format!("source.{}", naming::source(def_name))
}
