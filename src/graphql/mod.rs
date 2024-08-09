use crate::definitions::{
    Definition, Enum, EnumValueDefinition, FieldDefinition, InputFieldDefinition,
    Input, Interface, Module, Object,
    Scalar, Schema, Submodule, TypeExpression, Union,
};
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

    // format_schema_definition(def)

    for child in &s.children {
        schema_definitions.extend(build_schema_ast_from_module(s, &child));
    }
    for submodule in &s.submodules {
        schema_definitions.extend(build_schema_ast_from_submodule(s, &submodule));
    }
    return schema_definitions;
}

fn build_schema_ast_from_module<'a>(
    s: &'a Schema,
    module: &'a Module,
) -> Vec<schema::Definition<'a, &'a str>> {
    let mut schema_definitions = Vec::new();
    for child in &module.children {
        schema_definitions.extend(build_schema_ast_from_module(s, &child));
    }
    for submodule in &module.submodules {
        schema_definitions.extend(build_schema_ast_from_submodule(s, &submodule));
    }
    return schema_definitions;
}

fn build_schema_ast_from_submodule<'a>(
    s: &'a Schema,
    submodule: &'a Submodule,
) -> Vec<schema::Definition<'a, &'a str>> {
    let mut schema_definitions = Vec::new();

    for definition in &submodule.definitions {
        match definition {
            // ScalarTypeDefinition
            Definition::Scalar(def) => {
                schema_definitions.push(schema::Definition::TypeDefinition(
                    schema::TypeDefinition::Scalar(format_scalar_definition(s, def)),
                ));
            }
            // ObjectTypeDefinition
            Definition::Object(def) => {
                schema_definitions.push(schema::Definition::TypeDefinition(
                    schema::TypeDefinition::Object(format_object_definition(s, def)),
                ));
            }
            // InterfaceTypeDefinition
            Definition::Interface(def) => {
                schema_definitions.push(schema::Definition::TypeDefinition(
                    schema::TypeDefinition::Interface(format_interface_definition(s, def)),
                ));
            }
            // UnionTypeDefinition
            Definition::Union(def) => {
                schema_definitions.push(schema::Definition::TypeDefinition(
                    schema::TypeDefinition::Union(format_union_definition(s, def)),
                ));
            }
            // EnumTypeDefinition
            Definition::Enum(def) => {
                schema_definitions.push(schema::Definition::TypeDefinition(
                    schema::TypeDefinition::Enum(format_enum_definition(s, def)),
                ));
            }
            // InputObjectTypeDefinition
            Definition::Input(def) => {
                schema_definitions.push(schema::Definition::TypeDefinition(
                    schema::TypeDefinition::InputObject(format_input_definition(s, def)),
                ));
            }
            _ => {}
        }
    }

    return schema_definitions;
}

// fn format_schema_definition<'a>(s: &'a Schema) -> schema::SchemaDefinition<'a, &'a str> {
//     schema::SchemaDefinition {
//         position: graphql_parser::Pos::default(),
//         directives: vec![],
//         query: s.query.as_deref(),
//         mutation: s.mutation.as_deref(),
//         subscription: s.subscription.as_deref(),
//     }
// }

fn format_scalar_definition<'a>(
    s: &'a Schema,
    def: &'a Scalar,
) -> schema::ScalarType<'a, &'a str> {
    schema::ScalarType {
        position: graphql_parser::Pos::default(),
        name: def.name.as_str(),
        description: def.description.clone(),
        directives: vec![],
    }
}

fn format_object_definition<'a>(
    s: &'a Schema,
    def: &'a Object,
) -> schema::ObjectType<'a, &'a str> {
    schema::ObjectType {
        position: graphql_parser::Pos::default(),
        name: def.name.as_str(),
        description: def.description.clone(),
        implements_interfaces: def.interfaces.iter().map(|i| i.as_str()).collect(),
        directives: vec![],
        fields: def
            .fields
            .iter()
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
        implements_interfaces: def.interfaces.iter().map(|i| i.as_str()).collect(),
        directives: vec![],
        fields: def
            .fields
            .iter()
            .map(|f| format_field_definition(f))
            .collect(),
    }
}

fn format_union_definition<'a>(
    s: &'a Schema,
    def: &'a Union,
) -> schema::UnionType<'a, &'a str> {
    schema::UnionType {
        position: graphql_parser::Pos::default(),
        name: def.name.as_str(),
        description: def.description.clone(),
        directives: vec![],
        types: def.types.iter().map(|t| t.as_str()).collect(),
    }
}

fn format_enum_definition<'a>(
    s: &'a Schema,
    def: &'a Enum,
) -> schema::EnumType<'a, &'a str> {
    schema::EnumType {
        position: graphql_parser::Pos::default(),
        name: def.name.as_str(),
        description: def.description.clone(),
        directives: vec![],
        values: def
            .values
            .iter()
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
        fields: i
            .fields
            .iter()
            .map(|f| format_input_field_definition(f))
            .collect(),
    }
}

fn format_field_definition<'a>(f: &'a FieldDefinition) -> schema::Field<'a, &'a str> {
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

fn format_input_field_definition<'a>(
    f: &'a InputFieldDefinition,
) -> schema::InputValue<'a, &'a str> {
    schema::InputValue {
        position: graphql_parser::Pos::default(),
        name: f.name.as_str(),
        description: f.description.clone(),
        value_type: format_type_expression(&f.field_type),
        default_value: None, // TODO
        directives: vec![],
    }
}

fn format_enum_value_definition<'a>(v: &'a EnumValueDefinition) -> schema::EnumValue<'a, &'a str> {
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

// fn handle_object_extension<'a>(
//     def: &mut schema::ObjectType<'a, &'a str>,
//     ext: &'a ObjectTypeExtension,
// ) {
//     def.fields
//         .extend(ext.fields.iter().map(|f| format_field_definition(f)));
// }

// fn handle_interface_extension<'a>(
//     def: &mut schema::InterfaceType<'a, &'a str>,
//     ext: &'a InterfaceTypeExtension,
// ) {
//     def.fields
//         .extend(ext.fields.iter().map(|f| format_field_definition(f)));
// }

// fn handle_union_extension<'a>(
//     def: &mut schema::UnionType<'a, &'a str>,
//     ext: &'a UnionTypeExtension,
// ) {
//     let types: Vec<&str> = ext.types.iter().map(|t| t.as_str()).collect();
//     def.types.extend(types);
// }

// fn handle_enum_extension<'a>(def: &mut schema::EnumType<'a, &'a str>, ext: &'a EnumTypeExtension) {
//     def.values
//         .extend(ext.values.iter().map(|v| format_enum_value_definition(v)));
// }

// fn handle_input_extension<'a>(
//     def: &mut schema::InputObjectType<'a, &'a str>,
//     ext: &'a InputObjectTypeExtension,
// ) {
//     def.fields
//         .extend(ext.fields.iter().map(|f| format_input_field_definition(f)));
// }

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
