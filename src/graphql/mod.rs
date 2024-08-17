use crate::schema::{
    Definition, EnumDefinition, EnumValue, Field, InputDefinition, InputValue, InterfaceDefinition,
    ObjectDefinition, ScalarDefinition, Schema, TypeExpression, UnionDefinition, Value,
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

    for def in s.iter_definitions() {
        match def {
            Definition::ScalarDefinition(def) => {
                schema_definitions.push(schema::Definition::TypeDefinition(
                    schema::TypeDefinition::Scalar(format_scalar_definition(s, def)),
                ));
            }
            Definition::ObjectDefinition(def) => {
                schema_definitions.push(schema::Definition::TypeDefinition(
                    schema::TypeDefinition::Object(format_object_definition(s, def)),
                ));
            }
            Definition::InterfaceDefinition(def) => {
                schema_definitions.push(schema::Definition::TypeDefinition(
                    schema::TypeDefinition::Interface(format_interface_definition(s, def)),
                ));
            }
            Definition::UnionDefinition(def) => {
                schema_definitions.push(schema::Definition::TypeDefinition(
                    schema::TypeDefinition::Union(format_union_definition(s, def)),
                ));
            }
            Definition::EnumDefinition(def) => {
                schema_definitions.push(schema::Definition::TypeDefinition(
                    schema::TypeDefinition::Enum(format_enum_definition(s, def)),
                ));
            }
            Definition::InputDefinition(def) => {
                schema_definitions.push(schema::Definition::TypeDefinition(
                    schema::TypeDefinition::InputObject(format_input_definition(s, def)),
                ));
            }
            _ => {}
        }
    }

    return schema_definitions;
}

fn format_scalar_definition<'a>(
    s: &'a Schema,
    def: &'a ScalarDefinition,
) -> schema::ScalarType<'a, &'a str> {
    schema::ScalarType {
        position: graphql_parser::Pos::default(),
        name: def.name.as_str(),
        description: def.description.clone(),
        directives: format_directives(&None),
    }
}

fn format_object_definition<'a>(
    s: &'a Schema,
    def: &'a ObjectDefinition,
) -> schema::ObjectType<'a, &'a str> {
    schema::ObjectType {
        position: graphql_parser::Pos::default(),
        name: def.name.as_str(),
        description: def.description.clone(),
        implements_interfaces: s.collect_interfaces(&def.name),
        directives: format_directives(&None),
        fields: s
            .collect_fields(&def.name)
            .into_iter()
            .map(|f| format_field_definition(f))
            .collect(),
    }
}

fn format_interface_definition<'a>(
    s: &'a Schema,
    def: &'a InterfaceDefinition,
) -> schema::InterfaceType<'a, &'a str> {
    schema::InterfaceType {
        position: graphql_parser::Pos::default(),
        name: def.name.as_str(),
        description: def.description.clone(),
        implements_interfaces: s.collect_interfaces(&def.name),
        fields: s
            .collect_fields(&def.name)
            .into_iter()
            .map(|f| format_field_definition(f))
            .collect(),
        directives: format_directives(&None),
    }
}

fn format_union_definition<'a>(
    s: &'a Schema,
    def: &'a UnionDefinition,
) -> schema::UnionType<'a, &'a str> {
    schema::UnionType {
        position: graphql_parser::Pos::default(),
        name: def.name.as_str(),
        description: def.description.clone(),
        types: s.collect_union_types(&def.name),
        directives: format_directives(&None),
    }
}

fn format_enum_definition<'a>(
    s: &'a Schema,
    def: &'a EnumDefinition,
) -> schema::EnumType<'a, &'a str> {
    schema::EnumType {
        position: graphql_parser::Pos::default(),
        name: def.name.as_str(),
        description: def.description.clone(),
        directives: format_directives(&None),
        values: s
            .collect_enum_values(&def.name)
            .map(|v| format_enum_value_definition(v))
            .collect(),
    }
}

fn format_input_definition<'a>(
    s: &'a Schema,
    def: &'a InputDefinition,
) -> schema::InputObjectType<'a, &'a str> {
    schema::InputObjectType {
        position: graphql_parser::Pos::default(),
        name: def.name.as_str(),
        description: def.description.clone(),
        directives: format_directives(&None),
        fields: s
            .collect_input_fields(&def.name)
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
        directives: format_directives(&f.deprecation_reason),
    }
}

fn format_input_field_definition<'a>(f: &'a InputValue) -> schema::InputValue<'a, &'a str> {
    schema::InputValue {
        position: graphql_parser::Pos::default(),
        name: f.name.as_str(),
        description: f.description.clone(),
        value_type: format_type_expression(&f.field_type),
        default_value: f.default_value.as_ref().map(|v| format_value(v)),
        directives: format_directives(&f.deprecation_reason),
    }
}

fn format_enum_value_definition<'a>(v: &'a EnumValue) -> schema::EnumValue<'a, &'a str> {
    schema::EnumValue {
        position: graphql_parser::Pos::default(),
        name: v.name.as_str(),
        description: v.description.clone(),
        directives: format_directives(&v.deprecation_reason),
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

fn format_directives<'a>(
    deprecated_reason: &Option<String>,
) -> Vec<schema::Directive<'a, &'a str>> {
    let mut formatted_directives = vec![];

    if let Some(reason) = deprecated_reason {
        formatted_directives.push(schema::Directive {
            name: "deprecated",
            arguments: vec![("reason", schema::Value::String(reason.to_owned()))],
            position: graphql_parser::Pos::default(),
        });
    }

    return formatted_directives;
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
