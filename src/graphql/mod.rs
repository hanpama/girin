use crate::schema::{
    Definition, Enum, EnumValue, Field, Input, InputValue, Interface, Object, Scalar, Schema,
    TypeExpression, Union, Value,
};
use graphql_parser::query::Number;
use graphql_parser::schema;
use std::error::Error;
use std::{fs::File, io::Write, path::PathBuf};

pub fn render(s: &Schema, outfile: PathBuf) -> Result<(), GraphQLRenderingError> {
    let schema_ast = build_schema_ast(s);

    let mut file = File::create(outfile)?;
    file.write_all(&schema_ast.to_string().as_bytes())?;

    return Ok(());
}

fn build_schema_ast<'a>(s: &'a Schema) -> schema::Document<'a, &'a str> {
    return schema::Document {
        definitions: build_schema_ast_from_schema(s),
    };
}

fn build_schema_ast_from_schema<'a>(s: &'a Schema) -> Vec<schema::Definition<'a, &'a str>> {
    let mut schema_definitions = Vec::new();

    for definition in s.iter_definitions() {
        match definition {
            Definition::Scalar(def) => {
                schema_definitions.push(schema::Definition::TypeDefinition(
                    schema::TypeDefinition::Scalar(format_scalar_definition(s, def)),
                ));
            }
            Definition::Object(def) => {
                schema_definitions.push(schema::Definition::TypeDefinition(
                    schema::TypeDefinition::Object(format_object_definition(s, def)),
                ));
            }
            Definition::Interface(def) => {
                schema_definitions.push(schema::Definition::TypeDefinition(
                    schema::TypeDefinition::Interface(format_interface_definition(s, def)),
                ));
            }
            Definition::Union(def) => {
                schema_definitions.push(schema::Definition::TypeDefinition(
                    schema::TypeDefinition::Union(format_union_definition(s, def)),
                ));
            }
            Definition::Enum(def) => {
                schema_definitions.push(schema::Definition::TypeDefinition(
                    schema::TypeDefinition::Enum(format_enum_definition(s, def)),
                ));
            }
            Definition::Input(def) => {
                schema_definitions.push(schema::Definition::TypeDefinition(
                    schema::TypeDefinition::InputObject(format_input_definition(s, def)),
                ));
            }
        }
    }

    return schema_definitions;
}

fn format_scalar_definition<'a>(s: &'a Schema, def: &'a Scalar) -> schema::ScalarType<'a, &'a str> {
    schema::ScalarType {
        position: graphql_parser::Pos::default(),
        name: def.name.as_str(),
        description: def.description.clone(),
        directives: vec![],
    }
}

fn format_object_definition<'a>(s: &'a Schema, def: &'a Object) -> schema::ObjectType<'a, &'a str> {
    schema::ObjectType {
        position: graphql_parser::Pos::default(),
        name: def.name.as_str(),
        description: def.description.clone(),
        implements_interfaces: s
            .collect_object_interfaces(def)
            .map(|i| i.as_str())
            .collect(),
        directives: vec![],
        fields: s
            .collect_object_fields(def)
            .map(|f| format_field_definition(f))
            .collect(),
    }
}

fn format_interface_definition<'a>(
    s: &'a Schema,
    def: &'a Interface,
) -> schema::InterfaceType<'a, &'a str> {
    schema::InterfaceType {
        position: graphql_parser::Pos::default(),
        name: def.name.as_str(),
        description: def.description.clone(),
        implements_interfaces: s
            .collect_interface_interfaces(def)
            .map(|i| i.as_str())
            .collect(),
        directives: vec![],
        fields: s
            .collect_interface_fields(def)
            .map(|f| format_field_definition(f))
            .collect(),
    }
}

fn format_union_definition<'a>(s: &'a Schema, def: &'a Union) -> schema::UnionType<'a, &'a str> {
    schema::UnionType {
        position: graphql_parser::Pos::default(),
        name: def.name.as_str(),
        description: def.description.clone(),
        types: s.collect_possible_types(def).map(|t| t.as_str()).collect(),
        directives: vec![],
    }
}

fn format_enum_definition<'a>(s: &'a Schema, def: &'a Enum) -> schema::EnumType<'a, &'a str> {
    schema::EnumType {
        position: graphql_parser::Pos::default(),
        name: def.name.as_str(),
        description: def.description.clone(),
        directives: vec![],
        values: s
            .collect_enum_values(def)
            .map(|v| format_enum_value_definition(v))
            .collect(),
    }
}

fn format_input_definition<'a>(
    s: &'a Schema,
    i: &'a Input,
) -> schema::InputObjectType<'a, &'a str> {
    schema::InputObjectType {
        position: graphql_parser::Pos::default(),
        name: i.name.as_str(),
        description: i.description.clone(),
        directives: vec![],
        fields: s
            .collect_input_fields(i)
            .map(|f| format_input_field_definition(f))
            .collect(),
    }
}

fn format_field_definition<'a>(f: &'a Field) -> schema::Field<'a, &'a str> {
    schema::Field {
        position: graphql_parser::Pos::default(),
        name: f.name.as_str(),
        description: f.description.clone(),
        arguments: f
            .args
            .iter()
            .map(|a| format_input_field_definition(a))
            .collect(),
        field_type: format_type_expression(&f.field_type),
        directives: vec![],
    }
}

fn format_input_field_definition<'a>(f: &'a InputValue) -> schema::InputValue<'a, &'a str> {
    schema::InputValue {
        position: graphql_parser::Pos::default(),
        name: f.name.as_str(),
        description: f.description.clone(),
        value_type: format_type_expression(&f.field_type),
        default_value: f.default_value.as_ref().map(|v| format_value(v)),
        directives: vec![],
    }
}

fn format_enum_value_definition<'a>(v: &'a EnumValue) -> schema::EnumValue<'a, &'a str> {
    schema::EnumValue {
        position: graphql_parser::Pos::default(),
        name: v.name.as_str(),
        description: v.description.clone(),
        directives: vec![],
    }
}

fn format_type_expression<'a>(t: &'a TypeExpression) -> schema::Type<'a, &'a str> {
    match t {
        TypeExpression::NamedType(n) => schema::Type::NamedType(n.as_str()),
        TypeExpression::ListType(l) => schema::Type::ListType(Box::new(format_type_expression(l))),
        TypeExpression::NonNullType(n) => {
            schema::Type::NonNullType(Box::new(format_type_expression(n)))
        }
    }
}

fn format_value<'a>(t: &'a Value) -> schema::Value<'a, &'a str> {
    match t {
        Value::Int(i) => schema::Value::Int(Number::from(*i)),
        Value::Float(f) => schema::Value::Float(*f),
        Value::String(s) => schema::Value::String(s.to_owned()),
        Value::Boolean(b) => schema::Value::Boolean(*b),
        Value::Null => schema::Value::Null,
        Value::Enum(e) => schema::Value::Enum(e.as_str()),
        Value::List(l) => schema::Value::List(l.iter().map(|v| format_value(v)).collect()),
        Value::Object(o) => schema::Value::Object(
            o.iter()
                .map(|(k, v)| (k.as_str(), format_value(v)))
                .collect(),
        ),
    }
}

#[derive(Debug)]
pub enum GraphQLRenderingError {
    IoError(std::io::Error),
}

impl Error for GraphQLRenderingError {}

impl From<std::io::Error> for GraphQLRenderingError {
    fn from(e: std::io::Error) -> Self {
        GraphQLRenderingError::IoError(e)
    }
}

impl std::fmt::Display for GraphQLRenderingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GraphQLRenderingError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}
