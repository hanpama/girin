use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::{fs::File, io::Read};

use graphql_parser::parse_schema;
use graphql_parser::schema;

use crate::model::{
    Definition, EnumTypeDefinition, EnumTypeExtension, EnumValueDefinition, FieldDefinition,
    InputFieldDefinition, InputObjectTypeDefinition, InputObjectTypeExtension,
    InterfaceTypeDefinition, InterfaceTypeExtension, Module, ObjectTypeDefinition,
    ObjectTypeExtension, Position, ResolverConfig, ScalarTypeDefinition, Schema, SchemaDefinition,
    SourceConfig, Submodule, TypeExpression, UnionTypeDefinition, UnionTypeExtension, Value,
};

pub fn build_schema(dir_path: &PathBuf) -> Result<Schema, SchemaBuildingError> {
    let module = build_module(dir_path)?;
    return Ok(Schema {
        root_module: module,
    });
}

fn build_module(dir_path: &PathBuf) -> Result<Module, SchemaBuildingError> {
    let read_dir = read_dir(&dir_path)?;

    let mut module: Module = Module {
        path: dir_path.to_str().unwrap().to_string(),
        name: dir_path.file_stem().unwrap().to_str().unwrap().to_string(),
        children: Vec::new(),
        submodules: Vec::new(),
    };
    for item in read_dir {
        let item = item?;
        let file_type = item.file_type()?;

        if file_type.is_dir() {
            let child = build_module(&item.path())?;
            module.children.push(child);
        } else {
            let submodule = build_submodule(&item.path())?;
            module.submodules.push(submodule);
        }
    }

    return Ok(module);
}

fn build_submodule(file_path: &Path) -> Result<Submodule, SchemaBuildingError> {
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
        smod.definitions.push(build_definition(&smod, def)?);
    }
    return Ok(smod);
}

fn build_definition(
    smod: &Submodule,
    def: schema::Definition<String>,
) -> Result<Definition, SchemaBuildingError> {
    match def {
        schema::Definition::SchemaDefinition(def) => Ok(Definition::SchemaDefinition(
            build_schema_definition(smod, &def),
        )),
        schema::Definition::TypeDefinition(def) => match def {
            schema::TypeDefinition::Scalar(def) => Ok(Definition::ScalarTypeDefinition(
                build_scalar_type_definition(smod, &def)?,
            )),
            schema::TypeDefinition::Object(def) => Ok(Definition::ObjectTypeDefinition(
                build_object_type_definition(smod, &def)?,
            )),
            schema::TypeDefinition::Interface(def) => Ok(Definition::InterfaceTypeDefinition(
                build_interface_type_definition(smod, &def)?,
            )),
            schema::TypeDefinition::Union(def) => Ok(Definition::UnionTypeDefinition(
                build_union_type_definition(smod, &def)?,
            )),
            schema::TypeDefinition::Enum(def) => Ok(Definition::EnumTypeDefinition(
                build_enum_type_definition(smod, &def)?,
            )),
            schema::TypeDefinition::InputObject(def) => Ok(Definition::InputObjectTypeDefinition(
                build_input_object_type_definition(smod, &def)?,
            )),
        },
        schema::Definition::TypeExtension(def) => match def {
            schema::TypeExtension::Object(def) => Ok(Definition::ObjectTypeExtension(
                build_object_type_extension(smod, &def)?,
            )),
            schema::TypeExtension::Enum(def) => Ok(Definition::EnumTypeExtension(
                build_enum_type_extension(smod, &def)?,
            )),
            schema::TypeExtension::InputObject(def) => Ok(Definition::InputObjectTypeExtension(
                build_input_object_type_extension(smod, &def)?,
            )),
            schema::TypeExtension::Interface(def) => Ok(Definition::InterfaceTypeExtension(
                build_interface_extension(smod, &def)?,
            )),
            schema::TypeExtension::Union(def) => Ok(Definition::UnionTypeExtension(
                build_union_type_extension(smod, &def)?,
            )),
            schema::TypeExtension::Scalar(def) => {
                todo!()
            }
        },
        schema::Definition::DirectiveDefinition(def) => {
            todo!()
        }
    }
}

fn build_schema_definition(
    smod: &Submodule,
    def: &schema::SchemaDefinition<String>,
) -> SchemaDefinition {
    return SchemaDefinition {
        query: def.query.clone(),
        mutation: def.mutation.clone(),
        subscription: def.subscription.clone(),
    };
}

fn build_scalar_type_definition(
    smod: &Submodule,
    def: &schema::ScalarType<String>,
) -> Result<ScalarTypeDefinition, SchemaBuildingError> {
    let mut type_aliases: HashMap<String, String> = HashMap::new();
    for dir in def.directives.iter() {
        match dir.name.as_str() {
            "typeAlias" => {
                let map = handle_source_type(&smod, &dir)?;
                for (key, value) in map.iter() {
                    type_aliases.insert(key.clone(), value.clone());
                }
            }
            _ => {
                // return Err(SchemaBuildingError::UnsupportedDirectiveError {
                //     directive_name: dir.name.clone(),
                // });
            }
        }
    }
    return Ok(ScalarTypeDefinition {
        name: def.name.clone(),
        description: def.description.clone(),
        position: build_position(smod, &def.position),
        type_aliases,
    });
}

fn build_object_type_definition(
    smod: &Submodule,
    def: &schema::ObjectType<String>,
) -> Result<ObjectTypeDefinition, SchemaBuildingError> {
    let fields_result: Result<Vec<FieldDefinition>, SchemaBuildingError> = def
        .fields
        .iter()
        .map(|fdef| build_field_definition(smod, &fdef))
        .collect();
    return Ok(ObjectTypeDefinition {
        name: def.name.clone(),
        description: def.description.clone(),
        interfaces: def.implements_interfaces.clone(),
        fields: fields_result?,
        position: build_position(smod, &def.position),
    });
}

fn build_object_type_extension(
    smod: &Submodule,
    def: &schema::ObjectTypeExtension<String>,
) -> Result<ObjectTypeExtension, SchemaBuildingError> {
    let fields_result: Result<Vec<FieldDefinition>, SchemaBuildingError> = def
        .fields
        .iter()
        .map(|fdef| build_field_definition(smod, &fdef))
        .collect();
    return Ok(ObjectTypeExtension {
        name: def.name.clone(),
        interfaces: def.implements_interfaces.clone(),
        fields: fields_result?,
        position: build_position(smod, &def.position),
    });
}

fn build_field_definition(
    smod: &Submodule,
    def: &schema::Field<String>,
) -> Result<FieldDefinition, SchemaBuildingError> {
    let mut deprecation_reason: Option<String> = None;
    let mut resolve: Option<ResolverConfig> = None;
    let mut source_configs: Vec<SourceConfig> = Vec::new();
    for dir in def.directives.iter() {
        match dir.name.as_str() {
            "deprecated" => {
                deprecation_reason = Some(handle_deprecate(&smod, &dir)?);
            }
            "resolve" => {
                resolve = Some(handle_resolve(&smod, &dir)?);
            }
            "source" => {
                source_configs.push(handle_source(&smod, &dir)?);
            }
            _ => {
                // return Err(SchemaBuildingError::UnsupportedDirectiveError {
                //     directive_name: dir.name.clone(),
                // });
            }
        }
    }
    let args_result: Result<Vec<InputFieldDefinition>, SchemaBuildingError> = def
        .arguments
        .iter()
        .map(|def| build_input_field_definition(smod, &def))
        .collect();

    return Ok(FieldDefinition {
        name: def.name.clone(),
        description: def.description.clone(),
        deprecation_reason,
        args: args_result?,
        field_type: build_type_expression(smod, &def.field_type),
        resolve,
        source_configs,
        position: build_position(smod, &def.position),
    });
}

fn build_type_expression(smod: &Submodule, def: &schema::Type<String>) -> TypeExpression {
    match def {
        schema::Type::NamedType(name) => TypeExpression::NamedType(name.to_string()),
        schema::Type::ListType(inner) => {
            TypeExpression::ListType(Box::new(build_type_expression(smod, inner)))
        }
        schema::Type::NonNullType(inner) => {
            TypeExpression::NonNullType(Box::new(build_type_expression(smod, inner)))
        }
    }
}

fn build_interface_type_definition(
    smod: &Submodule,
    def: &schema::InterfaceType<String>,
) -> Result<InterfaceTypeDefinition, SchemaBuildingError> {
    let fields_result: Result<Vec<FieldDefinition>, SchemaBuildingError> = def
        .fields
        .iter()
        .map(|fdef| build_field_definition(smod, &fdef))
        .collect();
    return Ok(InterfaceTypeDefinition {
        name: def.name.clone(),
        description: def.description.clone(),
        interfaces: def.implements_interfaces.clone(),
        fields: fields_result?,
        position: build_position(smod, &def.position),
    });
}

fn build_interface_extension(
    smod: &Submodule,
    def: &schema::InterfaceTypeExtension<String>,
) -> Result<InterfaceTypeExtension, SchemaBuildingError> {
    let fields_result: Result<Vec<FieldDefinition>, SchemaBuildingError> = def
        .fields
        .iter()
        .map(|fdef| build_field_definition(smod, &fdef))
        .collect();
    return Ok(InterfaceTypeExtension {
        name: def.name.clone(),
        interfaces: def.implements_interfaces.clone(),
        fields: fields_result?,
        position: build_position(smod, &def.position),
    });
}

fn build_union_type_definition(
    smod: &Submodule,
    def: &schema::UnionType<String>,
) -> Result<UnionTypeDefinition, SchemaBuildingError> {
    return Ok(UnionTypeDefinition {
        name: def.name.clone(),
        description: def.description.clone(),
        types: def.types.clone(),
        position: build_position(smod, &def.position),
    });
}

fn build_union_type_extension(
    smod: &Submodule,
    def: &schema::UnionTypeExtension<String>,
) -> Result<UnionTypeExtension, SchemaBuildingError> {
    return Ok(UnionTypeExtension {
        name: def.name.clone(),
        types: def.types.clone(),
        position: build_position(smod, &def.position),
    });
}

fn build_enum_type_definition(
    smod: &Submodule,
    def: &schema::EnumType<String>,
) -> Result<EnumTypeDefinition, SchemaBuildingError> {
    let values_result: Result<Vec<EnumValueDefinition>, SchemaBuildingError> = def
        .values
        .iter()
        .map(|vdef| build_enum_value_definition(smod, &vdef))
        .collect();

    return Ok(EnumTypeDefinition {
        name: def.name.clone(),
        description: def.description.clone(),
        values: values_result?,
        position: build_position(smod, &def.position),
    });
}

fn build_enum_type_extension(
    smod: &Submodule,
    def: &schema::EnumTypeExtension<String>,
) -> Result<EnumTypeExtension, SchemaBuildingError> {
    let values_result: Result<Vec<EnumValueDefinition>, SchemaBuildingError> = def
        .values
        .iter()
        .map(|vdef| build_enum_value_definition(smod, &vdef))
        .collect();

    return Ok(EnumTypeExtension {
        name: def.name.clone(),
        values: values_result?,
        position: build_position(smod, &def.position),
    });
}

fn build_enum_value_definition(
    smod: &Submodule,
    def: &schema::EnumValue<String>,
) -> Result<EnumValueDefinition, SchemaBuildingError> {
    let mut deprecation_reason: Option<String> = None;

    for dir in def.directives.iter() {
        match dir.name.as_str() {
            "deprecated" => {
                deprecation_reason = Some(handle_deprecate(&smod, &dir)?);
            }
            _ => {
                // return Err(SchemaBuildingError::UnsupportedDirectiveError {
                //     directive_name: dir.name.clone(),
                // });
            }
        }
    }

    return Ok(EnumValueDefinition {
        name: def.name.clone(),
        description: def.description.clone(),
        deprecation_reason,
        position: build_position(smod, &def.position),
    });
}

fn build_input_object_type_definition(
    smod: &Submodule,
    def: &schema::InputObjectType<String>,
) -> Result<InputObjectTypeDefinition, SchemaBuildingError> {
    let fields_result: Result<Vec<InputFieldDefinition>, SchemaBuildingError> = def
        .fields
        .iter()
        .map(|fdef| build_input_field_definition(smod, &fdef))
        .collect();
    return Ok(InputObjectTypeDefinition {
        name: def.name.clone(),
        description: def.description.clone(),
        fields: fields_result?,
        position: build_position(smod, &def.position),
    });
}

fn build_input_object_type_extension(
    smod: &Submodule,
    def: &schema::InputObjectTypeExtension<String>,
) -> Result<InputObjectTypeExtension, SchemaBuildingError> {
    let fields_result: Result<Vec<InputFieldDefinition>, SchemaBuildingError> = def
        .fields
        .iter()
        .map(|fdef| build_input_field_definition(smod, &fdef))
        .collect();
    return Ok(InputObjectTypeExtension {
        name: def.name.clone(),
        fields: fields_result?,
        position: build_position(smod, &def.position),
    });
}

fn build_input_field_definition(
    smod: &Submodule,
    def: &schema::InputValue<String>,
) -> Result<InputFieldDefinition, SchemaBuildingError> {
    let mut deprecation_reason: Option<String> = None;

    for dir in def.directives.iter() {
        match dir.name.as_str() {
            "deprecated" => {
                deprecation_reason = Some(handle_deprecate(&smod, &dir)?);
            }
            _ => {
                // return Err(SchemaBuildingError::UnsupportedDirectiveError {
                //     directive_name: dir.name.clone(),
                // });
            }
        }
    }

    return Ok(InputFieldDefinition {
        name: def.name.clone(),
        description: def.description.clone(),
        field_type: build_type_expression(smod, &def.value_type),
        default_value: def.default_value.clone().map(build_value),
        position: build_position(smod, &def.position),
        deprecation_reason,
    });
}

fn build_value(def: schema::Value<String>) -> Value {
    match def {
        schema::Value::Variable(_) => unreachable!(),
        schema::Value::Int(v) => Value::Int(v.as_i64().unwrap()),
        schema::Value::Float(v) => Value::Float(v),
        schema::Value::String(v) => Value::String(v),
        schema::Value::Boolean(v) => Value::Boolean(v),
        schema::Value::Null => Value::Null,
        schema::Value::Enum(v) => Value::Enum(v),
        schema::Value::List(v) => Value::List(
            v.into_iter() //
                .map(build_value)
                .collect(),
        ),
        schema::Value::Object(v) => Value::Object(
            v.into_iter()
                .map(|(k, v)| (k, build_value(v)))
                .collect::<HashMap<String, Value>>(),
        ),
    }
}

fn handle_deprecate(
    smod: &Submodule,
    directive: &schema::Directive<String>,
) -> Result<String, SchemaBuildingError> {
    let mut reason = "No longer supported".to_string();
    let mut violations = vec![];

    for (key, value) in directive.arguments.iter() {
        match key.as_str() {
            "reason" => match value {
                schema::Value::String(value) => {
                    reason = value.clone();
                }
                _ => {
                    violations.push(GraphQLValidationViolation::ValuesOfCorrectType {
                        pos: build_position(smod, &directive.position),
                        name: key.clone(),
                        value: value.to_string(),
                    });
                }
            },
            _ => {
                violations.push(GraphQLValidationViolation::KnownArgumentNames {
                    pos: build_position(smod, &directive.position),
                    name: key.clone(),
                });
            }
        }
    }

    if !violations.is_empty() {
        return Err(SchemaBuildingError::GraphQLValidationViolation(violations));
    }

    return Ok(reason);
}

fn handle_resolve(
    smod: &Submodule,
    directive: &schema::Directive<String>,
) -> Result<ResolverConfig, SchemaBuildingError> {
    let mut def = ResolverConfig { sync: false };
    let mut violations = vec![];

    for (key, value) in directive.arguments.iter() {
        match key.as_str() {
            "sync" => match value {
                schema::Value::Boolean(value) => {
                    def.sync = value.clone();
                }
                _ => {
                    violations.push(GraphQLValidationViolation::ValuesOfCorrectType {
                        pos: build_position(smod, &directive.position),
                        name: key.clone(),
                        value: value.to_string(),
                    });
                }
            },
            _ => {
                violations.push(GraphQLValidationViolation::KnownArgumentNames {
                    pos: build_position(smod, &directive.position),
                    name: key.clone(),
                });
            }
        }
    }

    if !violations.is_empty() {
        return Err(SchemaBuildingError::GraphQLValidationViolation(violations));
    }

    return Ok(def);
}

fn handle_source(
    smod: &Submodule,
    directive: &schema::Directive<String>,
) -> Result<SourceConfig, SchemaBuildingError> {
    let mut name: Option<String> = None;
    let mut type_: Option<TypeExpression> = None;
    let mut violations = vec![];

    for (key, value) in directive.arguments.iter() {
        match key.as_str() {
            "name" => match value {
                schema::Value::String(value) => {
                    name = Some(value.clone());
                }
                _ => {
                    violations.push(GraphQLValidationViolation::ValuesOfCorrectType {
                        pos: build_position(smod, &directive.position),
                        name: key.clone(),
                        value: value.to_string(),
                    });
                }
            },
            "type" => match value {
                schema::Value::String(value) => {
                    type_ = Some(TypeExpression::NamedType(value.clone()));
                }
                _ => {
                    violations.push(GraphQLValidationViolation::ValuesOfCorrectType {
                        pos: build_position(smod, &directive.position),
                        name: key.clone(),
                        value: value.to_string(),
                    });
                }
            },
            _ => {
                violations.push(GraphQLValidationViolation::KnownArgumentNames {
                    pos: build_position(smod, &directive.position),
                    name: key.clone(),
                });
            }
        }
    }

    if name.is_none() {
        violations.push(GraphQLValidationViolation::ProvidedRequiredArguments {
            pos: build_position(smod, &directive.position),
            name: "name".to_string(),
        });
    }
    if type_.is_none() {
        violations.push(GraphQLValidationViolation::ProvidedRequiredArguments {
            pos: build_position(smod, &directive.position),
            name: "type".to_string(),
        });
    }

    if !violations.is_empty() {
        return Err(SchemaBuildingError::GraphQLValidationViolation(violations));
    }

    let def = SourceConfig {
        name: name.unwrap(),
        type_: type_.unwrap(),
    };

    return Ok(def);
}

fn handle_source_type(
    smod: &Submodule,
    directive: &schema::Directive<String>,
) -> Result<HashMap<String, String>, SchemaBuildingError> {
    let mut map = HashMap::new();
    let mut violations = vec![];

    for (key, value) in directive.arguments.iter() {
        match value {
            schema::Value::String(value) => {
                map.insert(key.clone(), value.clone());
            }
            _ => {
                // build_positioh
                violations.push(GraphQLValidationViolation::ValuesOfCorrectType {
                    pos: build_position(smod, &directive.position),
                    name: key.clone(),
                    value: value.to_string(),
                });
            }
        }
    }
    if !violations.is_empty() {
        return Err(SchemaBuildingError::GraphQLValidationViolation(violations));
    }

    return Ok(map);
}

fn build_position(smod: &Submodule, def: &graphql_parser::Pos) -> Position {
    return Position {
        file: smod.path.to_string(),
        line: def.line,
        column: def.column,
    };
}

#[derive(Debug)]
pub enum SchemaBuildingError {
    GraphQLParserError(graphql_parser::schema::ParseError),
    GraphQLValidationViolation(Vec<GraphQLValidationViolation>),
    IOError(std::io::Error),
}

impl fmt::Display for SchemaBuildingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SchemaBuildingError::GraphQLParserError(err) => {
                write!(f, "GraphQL Parser Error: {}", err)
            }
            SchemaBuildingError::GraphQLValidationViolation(err) => {
                write!(f, "GraphQL Validation Violation: {:?}", err)
            }
            SchemaBuildingError::IOError(err) => {
                write!(f, "IO Error: {}", err)
            }
        }
    }
}

impl Error for SchemaBuildingError {}

impl From<std::io::Error> for SchemaBuildingError {
    fn from(err: std::io::Error) -> Self {
        SchemaBuildingError::IOError(err)
    }
}

impl From<graphql_parser::schema::ParseError> for SchemaBuildingError {
    fn from(err: graphql_parser::schema::ParseError) -> Self {
        SchemaBuildingError::GraphQLParserError(err)
    }
}

#[derive(Debug)]
pub enum GraphQLValidationViolation {
    ProvidedRequiredArguments {
        pos: Position,
        name: String,
    },
    KnownArgumentNames {
        pos: Position,
        name: String,
    },
    ValuesOfCorrectType {
        pos: Position,
        name: String,
        value: String,
    },
    LoneSchemaDefinition {
        pos: Position,
    },
    PossibleTypeExtensions {
        pos: Position,
        def: String,
        ext: String,
    },
    UniqueArgumentDefinitionNames {
        pos: Position,
        name: String,
    },
    UniqueDirectiveNames {
        pos: Position,
        name: String,
    },
    UniqueEnumValueNames {
        pos: Position,
        name: String,
    },
    UniqueFieldDefinitionNames {
        pos: Position,
        name: String,
    },
    UniqueTypeNames {
        pos: Position,
        name: String,
    },
}

impl std::fmt::Display for GraphQLValidationViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GraphQLValidationViolation::ProvidedRequiredArguments { pos, name } => {
                write!(f, "Provided required arguments violation: {}", name)
            }
            GraphQLValidationViolation::KnownArgumentNames { pos, name } => {
                write!(f, "Known argument names violation: {}", name)
            }
            GraphQLValidationViolation::ValuesOfCorrectType { pos, name, value } => {
                write!(f, "Values of correct type violation: {} {}", name, value)
            }
            GraphQLValidationViolation::LoneSchemaDefinition { pos } => {
                write!(f, "Lone schema definition violation")
            }
            GraphQLValidationViolation::PossibleTypeExtensions { pos, def, ext } => {
                write!(f, "Possible type extensions violation")
            }
            GraphQLValidationViolation::UniqueArgumentDefinitionNames { pos, name } => {
                write!(f, "Unique argument definition names violation: {}", name)
            }
            GraphQLValidationViolation::UniqueDirectiveNames { pos, name } => {
                write!(f, "Unique directive names violation: {}", name)
            }
            GraphQLValidationViolation::UniqueEnumValueNames { pos, name } => {
                write!(f, "Unique enum value names violation: {}", name)
            }
            GraphQLValidationViolation::UniqueFieldDefinitionNames { pos, name } => {
                write!(f, "Unique field definition names violation: {}", name)
            }
            GraphQLValidationViolation::UniqueTypeNames { pos, name } => {
                write!(f, "Unique type names violation: {}", name)
            }
        }
    }
}
