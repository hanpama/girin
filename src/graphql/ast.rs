use crate::model::{
    Definition, EnumTypeDefinition, EnumTypeExtension, EnumValueDefinition, FieldDefinition,
    InputFieldDefinition, InputObjectTypeDefinition, InputObjectTypeExtension,
    InterfaceTypeDefinition, InterfaceTypeExtension, Module, ObjectTypeDefinition,
    ObjectTypeExtension, ScalarTypeDefinition, Schema, SchemaDefinition, Submodule, TypeExpression,
    UnionTypeDefinition, UnionTypeExtension,
};
use graphql_parser::schema;
use std::collections::HashMap;

pub fn build_schema_ast<'a>(s: &'a Schema) -> schema::Document<'a, &'a str> {
    let definitions = build_schema_ast_from_module(&s.root_module);
    return schema::Document {
        definitions: definitions.values().cloned().collect(),
    };
}

fn build_schema_ast_from_module<'a>(
    module: &'a Module,
) -> HashMap<&'a str, schema::Definition<'a, &'a str>> {
    let mut schema_definitions = HashMap::new();
    for child in &module.children {
        schema_definitions.extend(build_schema_ast_from_module(&child));
    }
    for submodule in &module.submodules {
        schema_definitions.extend(build_schema_ast_from_submodule(&submodule));
    }
    return schema_definitions;
}

fn build_schema_ast_from_submodule<'a>(
    submodule: &'a Submodule,
) -> HashMap<&'a str, schema::Definition<'a, &'a str>> {
    let mut schema_definitions = HashMap::new();

    for definition in &submodule.definitions {
        match definition {
            // SchemaDefinition
            Definition::SchemaDefinition(def) => {
                schema_definitions.insert(
                    "",
                    schema::Definition::SchemaDefinition(format_schema_definition(def)),
                );
            }
            // ScalarTypeDefinition
            Definition::ScalarTypeDefinition(def) => {
                schema_definitions.insert(
                    def.name.as_str(),
                    schema::Definition::TypeDefinition(schema::TypeDefinition::Scalar(
                        format_scalar_definition(def),
                    )),
                );
            }
            // ObjectTypeDefinition
            Definition::ObjectTypeDefinition(def) => {
                schema_definitions.insert(
                    def.name.as_str(),
                    schema::Definition::TypeDefinition(schema::TypeDefinition::Object(
                        format_object_definition(def),
                    )),
                );
            }
            // InterfaceTypeDefinition
            Definition::InterfaceTypeDefinition(def) => {
                schema_definitions.insert(
                    def.name.as_str(),
                    schema::Definition::TypeDefinition(schema::TypeDefinition::Interface(
                        format_interface_definition(def),
                    )),
                );
            }
            // UnionTypeDefinition
            Definition::UnionTypeDefinition(def) => {
                schema_definitions.insert(
                    def.name.as_str(),
                    schema::Definition::TypeDefinition(schema::TypeDefinition::Union(
                        format_union_definition(def),
                    )),
                );
            }
            // EnumTypeDefinition
            Definition::EnumTypeDefinition(def) => {
                schema_definitions.insert(
                    def.name.as_str(),
                    schema::Definition::TypeDefinition(schema::TypeDefinition::Enum(
                        format_enum_definition(def),
                    )),
                );
            }
            // InputObjectTypeDefinition
            Definition::InputObjectTypeDefinition(def) => {
                schema_definitions.insert(
                    def.name.as_str(),
                    schema::Definition::TypeDefinition(schema::TypeDefinition::InputObject(
                        format_input_object_definition(def),
                    )),
                );
            }
            _ => {}
        }
    }

    for definition in &submodule.definitions {
        use schema::Definition::TypeDefinition;
        use schema::TypeDefinition::{Enum, InputObject, Interface, Object, Union};

        match definition {
            // ObjectTypeExtension
            Definition::ObjectTypeExtension(ext) => {
                match schema_definitions.get_mut(ext.name.as_str()) {
                    Some(TypeDefinition(Object(def))) => {
                        handle_object_extension(def, ext);
                    }
                    _ => unreachable!(),
                }
            }
            // InterfaceTypeExtension
            Definition::InterfaceTypeExtension(ext) => {
                match schema_definitions.get_mut(ext.name.as_str()) {
                    Some(TypeDefinition(Interface(def))) => {
                        handle_interface_extension(def, ext);
                    }
                    _ => unreachable!(),
                }
            }
            // UnionTypeExtension
            Definition::UnionTypeExtension(ext) => {
                match schema_definitions.get_mut(ext.name.as_str()) {
                    Some(TypeDefinition(Union(def))) => {
                        handle_union_extension(def, ext);
                    }
                    _ => unreachable!(),
                }
            }
            // EnumTypeExtension
            Definition::EnumTypeExtension(ext) => {
                match schema_definitions.get_mut(ext.name.as_str()) {
                    Some(TypeDefinition(Enum(def))) => {
                        handle_enum_extension(def, ext);
                    }
                    _ => unreachable!(),
                }
            }
            // InputObjectTypeExtension
            Definition::InputObjectTypeExtension(ext) => {
                match schema_definitions.get_mut(ext.name.as_str()) {
                    Some(TypeDefinition(InputObject(def))) => {
                        handle_input_object_extension(def, ext);
                    }
                    _ => unreachable!(),
                }
            }
            _ => {}
        }
    }

    return schema_definitions;
}

fn format_schema_definition<'a>(s: &'a SchemaDefinition) -> schema::SchemaDefinition<'a, &'a str> {
    schema::SchemaDefinition {
        position: graphql_parser::Pos::default(),
        directives: vec![],
        query: s.query.as_deref(),
        mutation: s.mutation.as_deref(),
        subscription: s.subscription.as_deref(),
    }
}

fn format_scalar_definition<'a>(s: &'a ScalarTypeDefinition) -> schema::ScalarType<'a, &'a str> {
    schema::ScalarType {
        position: graphql_parser::Pos::default(),
        name: s.name.as_str(),
        description: s.description.clone(),
        directives: vec![],
    }
}

fn format_object_definition<'a>(o: &'a ObjectTypeDefinition) -> schema::ObjectType<'a, &'a str> {
    schema::ObjectType {
        position: graphql_parser::Pos::default(),
        name: o.name.as_str(),
        description: o.description.clone(),
        implements_interfaces: o.interfaces.iter().map(|i| i.as_str()).collect(),
        directives: vec![],
        fields: o
            .fields
            .iter()
            .map(|f| format_field_definition(f))
            .collect(),
    }
}

fn format_interface_definition<'a>(
    i: &'a InterfaceTypeDefinition,
) -> schema::InterfaceType<'a, &'a str> {
    schema::InterfaceType {
        position: graphql_parser::Pos::default(),
        name: i.name.as_str(),
        description: i.description.clone(),
        implements_interfaces: i.interfaces.iter().map(|i| i.as_str()).collect(),
        directives: vec![],
        fields: i
            .fields
            .iter()
            .map(|f| format_field_definition(f))
            .collect(),
    }
}

fn format_union_definition<'a>(u: &'a UnionTypeDefinition) -> schema::UnionType<'a, &'a str> {
    schema::UnionType {
        position: graphql_parser::Pos::default(),
        name: u.name.as_str(),
        description: u.description.clone(),
        directives: vec![],
        types: u.types.iter().map(|t| t.as_str()).collect(),
    }
}

fn format_enum_definition<'a>(e: &'a EnumTypeDefinition) -> schema::EnumType<'a, &'a str> {
    schema::EnumType {
        position: graphql_parser::Pos::default(),
        name: e.name.as_str(),
        description: e.description.clone(),
        directives: vec![],
        values: e
            .values
            .iter()
            .map(|v| format_enum_value_definition(v))
            .collect(),
    }
}

fn format_input_object_definition<'a>(
    i: &'a InputObjectTypeDefinition,
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

fn handle_object_extension<'a>(
    def: &mut schema::ObjectType<'a, &'a str>,
    ext: &'a ObjectTypeExtension,
) {
    def.fields
        .extend(ext.fields.iter().map(|f| format_field_definition(f)));
}

fn handle_interface_extension<'a>(
    def: &mut schema::InterfaceType<'a, &'a str>,
    ext: &'a InterfaceTypeExtension,
) {
    def.fields
        .extend(ext.fields.iter().map(|f| format_field_definition(f)));
}

fn handle_union_extension<'a>(
    def: &mut schema::UnionType<'a, &'a str>,
    ext: &'a UnionTypeExtension,
) {
    let types: Vec<&str> = ext.types.iter().map(|t| t.as_str()).collect();
    def.types.extend(types);
}

fn handle_enum_extension<'a>(def: &mut schema::EnumType<'a, &'a str>, ext: &'a EnumTypeExtension) {
    def.values
        .extend(ext.values.iter().map(|v| format_enum_value_definition(v)));
}

fn handle_input_object_extension<'a>(
    def: &mut schema::InputObjectType<'a, &'a str>,
    ext: &'a InputObjectTypeExtension,
) {
    def.fields
        .extend(ext.fields.iter().map(|f| format_input_field_definition(f)));
}
