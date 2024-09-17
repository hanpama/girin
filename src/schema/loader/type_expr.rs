use std::{iter::Peekable, str::Chars};

use crate::schema::TypeExpression;

pub fn parse_type_expression(string: &str) -> TypeExpression {
    let mut chars = string.chars().peekable();
    parse_type_expression_internal(&mut chars)
}

fn parse_type_expression_internal(chars: &mut Peekable<Chars>) -> TypeExpression {
    let type_expr: TypeExpression;
    if let Some(&'[') = chars.peek() {
        // Parse List Type: [Type]
        chars.next(); // Skip '['
        let inner_type = parse_type_expression_internal(chars);
        chars.next(); // Skip ']'
        type_expr = TypeExpression::ListType(Box::new(inner_type));
    } else {
        // Parse Named Type
        let mut name = String::new();
        while let Some(&ch) = chars.peek() {
            if ch == '!' || ch == ']' {
                break;
            }
            name.push(ch);
            chars.next();
        }

        type_expr = TypeExpression::NamedType(name.trim().to_string());
    }
    // Check if it's a Non-Null Type
    if let Some(&'!') = chars.peek() {
        chars.next(); // Skip '!'
        TypeExpression::NonNullType(Box::new(type_expr))
    } else {
        type_expr
    }
}

#[test]
fn test_parse_and_render_type_expression() {
    fn render_type_expression(type_expr: &TypeExpression) -> String {
        match type_expr {
            TypeExpression::NamedType(name) => name.clone(),
            TypeExpression::ListType(inner) => format!("[{}]", render_type_expression(inner)),
            TypeExpression::NonNullType(inner) => format!("{}!", render_type_expression(inner)),
        }
    }
    assert_eq!(
        render_type_expression(&parse_type_expression("String")),
        "String"
    );
    assert_eq!(
        render_type_expression(&parse_type_expression("String!")),
        "String!"
    );
    assert_eq!(
        render_type_expression(&parse_type_expression("[String!]")),
        "[String!]"
    );
    assert_eq!(
        render_type_expression(&parse_type_expression("[String]!")),
        "[String]!"
    );
    assert_eq!(
        render_type_expression(&parse_type_expression("[String!]!")),
        "[String!]!"
    );
    assert_eq!(
        render_type_expression(&parse_type_expression("[[String]]")),
        "[[String]]"
    );
    assert_eq!(
        render_type_expression(&parse_type_expression("[[String]]!")),
        "[[String]]!"
    );
    assert_eq!(
        render_type_expression(&parse_type_expression("[[String]!]")),
        "[[String]!]"
    );
    assert_eq!(
        render_type_expression(&parse_type_expression("[[String!]]")),
        "[[String!]]"
    );
    assert_eq!(
        render_type_expression(&parse_type_expression("[[String!]!]")),
        "[[String!]!]"
    );
    assert_eq!(
        render_type_expression(&parse_type_expression("[[String!]]!")),
        "[[String!]]!"
    );
    assert_eq!(
        render_type_expression(&parse_type_expression("[[String]!]!")),
        "[[String]!]!"
    );
    assert_eq!(
        render_type_expression(&parse_type_expression("[[String!]!]!")),
        "[[String!]!]!"
    );
}
