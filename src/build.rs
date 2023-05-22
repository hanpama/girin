use std::error::Error;
use std::fmt;
use std::fs::read_dir;
use std::path::Path;
use std::{fs::File, io::Read};

use graphql_parser::parse_schema;
use graphql_parser::schema;

use crate::model::{
    Definition, EnumTypeDefinition, FieldDefinition, InputFieldDefinition,
    InputObjectTypeDefinition, InterfaceTypeDefinition, Module, ObjectTypeDefinition, Position,
    ResolverDefinition, ScalarTypeDefinition, SchemaDefinition, Submodule, UnionTypeDefinition,
};

fn build_module(dir_path: &Path) -> Result<Module, Box<dyn Error>> {
    let read_dir = read_dir(&dir_path)?;

    let module: Module = Module {
        path: dir_path.to_str().unwrap().to_string(),
        name: dir_path.file_stem().unwrap().to_str().unwrap().to_string(),
        children: Vec::new(),
        submodules: Vec::new(),
    };
    for item in read_dir {
        let item = item?;
        let file_type = item.file_type()?;

        if file_type.is_dir() {
            build_module(&item.path());
        } else {
            build_submodule(&item.path());
        }
    }

    return Ok(module);
}

fn build_submodule(file_path: &Path) -> Result<Submodule, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let mut smod = Submodule {
        path: file_path.to_str().unwrap().to_string(),
        name: file_path.file_stem().unwrap().to_str().unwrap().to_string(),
        definitions: Vec::new(),
    };
    let document = parse_schema::<String>(&buf)?;
    for def in document.definitions {
        smod.definitions.push(build_definition(&smod, def));
    }
    return Ok(smod);
}

fn build_definition(smod: &Submodule, def: schema::Definition<String>) -> Definition {
    match def {
        schema::Definition::SchemaDefinition(def) => {
            Definition::SchemaDefinition(build_schema_definition(smod, def))
        }
        schema::Definition::TypeDefinition(def) => build_type_definition(smod, def),
        schema::Definition::TypeExtension(def) => {
            todo!()
        }
        schema::Definition::DirectiveDefinition(def) => {
            todo!()
        }
    }
}

fn build_type_definition(smod: &Submodule, def: schema::TypeDefinition<String>) -> Definition {
    match def {
        schema::TypeDefinition::Scalar(def) => {
            Definition::ScalarTypeDefinition(build_scalar_type_definition(smod, def))
        }
        schema::TypeDefinition::Object(def) => {
            Definition::ObjectTypeDefinition(build_object_type_definition(smod, def))
        }
        schema::TypeDefinition::Interface(def) => {
            Definition::InterfaceTypeDefinition(build_interface_type_definition(smod, def))
        }
        schema::TypeDefinition::Union(def) => {
            Definition::UnionTypeDefinition(build_union_type_definition(smod, def))
        }
        schema::TypeDefinition::Enum(def) => {
            Definition::EnumTypeDefinition(build_enum_type_definition(smod, def))
        }
        schema::TypeDefinition::InputObject(def) => {
            Definition::InputObjectTypeDefinition(build_input_object_type_definition(smod, def))
        }
        _ => {
            panic!();
        }
    }
}

fn build_schema_definition(
    smod: &Submodule,
    def: schema::SchemaDefinition<String>,
) -> SchemaDefinition {
    return SchemaDefinition {
        query: def.query,
        mutation: def.mutation,
        subscription: def.subscription,
    };
}

fn build_scalar_type_definition(
    smod: &Submodule,
    def: schema::ScalarType<String>,
) -> ScalarTypeDefinition {
    return ScalarTypeDefinition {
        name: def.name,
        description: def.description,
        position: build_position(smod.path.to_string(), def.position),
    };
}

fn build_object_type_definition(
    smod: &Submodule,
    def: schema::ObjectType<String>,
) -> ObjectTypeDefinition {
    return ObjectTypeDefinition {
        name: def.name,
        description: def.description,
        interfaces: def.implements_interfaces,
        fields: def
            .fields
            .into_iter()
            .map(|def| build_field_definition(smod, def))
            .collect(),
        position: build_position(smod.path.to_string(), def.position),
    };
}

fn build_field_definition(
    smod: &Submodule,
    def: schema::Field<String>,
) -> Result<FieldDefinition, SchemaParsingError> {
    let mut deprecation_reason: Option<String> = None;
    let mut resolve: Option<ResolverDefinition> = None;
    for dir in def.directives {
        match dir.name.as_str() {
            "deprecated" => {
                deprecation_reason = Some(handle_deprecate(dir)?);
            }
            "resolve" => {}
            _ => {
                return Err(SchemaParsingError {
                    message: format!("Unsupported directive: {}", dir.name.as_str()),
                });
            }
        }
    }
    return Ok(FieldDefinition {
        name: def.name,
        description: def.description,
        deprecation_reason: deprecation_reason,
        args: def
            .arguments
            .into_iter()
            .map(|def| build_input_field_definition(smod, def))
            .collect(),
        field_type: todo!(),
        resolve: todo!(),
        position: todo!(),
    });
    // todo!()
}

fn build_field_directives(smod: &Submodule, def: schema::Directive<String>) {}

fn build_interface_type_definition(
    smod: &Submodule,
    def: schema::InterfaceType<String>,
) -> InterfaceTypeDefinition {
    todo!()
}

fn build_union_type_definition(
    smod: &Submodule,
    def: schema::UnionType<String>,
) -> UnionTypeDefinition {
    todo!()
}

fn build_enum_type_definition(
    smod: &Submodule,
    def: schema::EnumType<String>,
) -> EnumTypeDefinition {
    todo!()
}

fn build_input_object_type_definition(
    smod: &Submodule,
    def: schema::InputObjectType<String>,
) -> InputObjectTypeDefinition {
    todo!()
}

fn build_input_field_definition(
    smod: &Submodule,
    def: schema::InputValue<String>,
) -> InputFieldDefinition {
    todo!()
}

fn build_position(file_path: String, def: graphql_parser::Pos) -> Position {
    return Position {
        file: file_path,
        line: def.line,
        column: def.column,
    };
}

fn handle_deprecate(directive: schema::Directive<String>) -> Result<String, SchemaParsingError> {
    let mut reason = "No longer supported".to_string();

    for (key, value) in directive.arguments {
        match key.as_str() {
            "reason" => match value {
                schema::Value::String(value) => {
                    reason = value;
                }
                _ => {
                    return Err(SchemaParsingError {
                        message: format!("Invalid argument type for @deprecated reason: {}", value),
                    })
                }
            },
            _ => {
                return Err(SchemaParsingError {
                    message: format!("Invalid argument for @deprecated: '{}'", key),
                })
            }
        }
    }

    return Ok(reason);
}

#[derive(Debug, Clone)]
pub struct SchemaParsingError {
    message: String,
}

impl fmt::Display for SchemaParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
