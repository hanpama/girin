use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::read_dir;
use std::path::Path;
use std::{fs::File, io::Read};

use graphql_parser::parse_schema;
use graphql_parser::schema;

use crate::schema::{
    Definition, Enum, EnumExtension, EnumValue, Extension, Field, Input, InputExtension,
    InputValue, Interface, InterfaceTypeExtension, Object, ObjectExtension, Position,
    ResolverConfig, Scalar, Schema, SourceConfig, TypeExpression, Union, UnionExtension, Value,
};

pub fn build_schema(dir: &Path) -> Result<Schema, SchemaBuildingError> {
    let mut schema = Schema::new();
    // PathBuf::from("value")
    build_directory(&mut schema, &dir, &dir)?;
    Ok(schema)
}

fn build_directory(s: &mut Schema, root: &Path, dir: &Path) -> Result<(), SchemaBuildingError> {
    let read_dir = read_dir(&dir)?;

    for item in read_dir {
        let item = item?;
        let file_type = item.file_type()?;

        if file_type.is_dir() {
            build_directory(s, &root, &item.path())?
        } else {
            build_file(s, &root, &item.path())?;
        }
    }

    Ok(())
}

fn build_file(s: &mut Schema, root: &Path, file: &Path) -> Result<(), SchemaBuildingError> {
    let mut buf = String::new();
    File::open(file)?.read_to_string(&mut buf)?;

    let document = parse_schema::<String>(&buf)?;
    let path = file.with_extension("");
    let path = path.strip_prefix(root).unwrap();

    let mut violations = vec![];
    for def in document.definitions {
        match def {
            schema::Definition::SchemaDefinition(def) => {
                s.query = def.query.clone();
                s.mutation = def.mutation.clone();
                s.subscription = def.subscription.clone();
            }
            schema::Definition::TypeDefinition(def) => match build_type_definition(def) {
                Ok(def) => s.add_definition(path, def),
                Err(err) => violations.extend(err.violations),
            },
            schema::Definition::TypeExtension(def) => match build_type_extension(def) {
                Ok(def) => s.add_extension(path, def),
                Err(err) => violations.extend(err.violations),
            },
            schema::Definition::DirectiveDefinition(def) => {
                todo!()
            }
        }
    }

    if !violations.is_empty() {
        return Err(SchemaBuildingError::GraphQLSchemaValidationError(
            GraphQLSchemaValidationError { violations },
        ));
    }

    Ok(())
}

fn build_type_definition(
    def: schema::TypeDefinition<String>,
) -> Result<Definition, GraphQLSchemaValidationError> {
    Ok(match def {
        schema::TypeDefinition::Scalar(def) => build_scalar_type_definition(&def)?.into(),
        schema::TypeDefinition::Object(def) => build_object_type_definition(&def)?.into(),
        schema::TypeDefinition::Interface(def) => build_interface_type_definition(&def)?.into(),
        schema::TypeDefinition::Union(def) => build_union_type_definition(&def)?.into(),
        schema::TypeDefinition::Enum(def) => build_enum_type_definition(&def)?.into(),
        schema::TypeDefinition::InputObject(def) => build_input_type_definition(&def)?.into(),
    })
}

fn build_type_extension(
    def: schema::TypeExtension<String>,
) -> Result<Extension, GraphQLSchemaValidationError> {
    Ok(match def {
        schema::TypeExtension::Object(def) => build_object_type_extension(&def)?.into(),
        schema::TypeExtension::Enum(def) => build_enum_type_extension(&def)?.into(),
        schema::TypeExtension::InputObject(def) => build_input_type_extension(&def)?.into(),
        schema::TypeExtension::Interface(def) => build_interface_extension(&def)?.into(),
        schema::TypeExtension::Union(def) => build_union_type_extension(&def)?.into(),
        schema::TypeExtension::Scalar(def) => {
            todo!()
        }
    })
}

fn build_scalar_type_definition(
    def: &schema::ScalarType<String>,
) -> Result<Scalar, GraphQLSchemaValidationError> {
    let mut violations = vec![];
    let mut type_aliases = HashMap::<String, String>::new();

    for dir in def.directives.iter() {
        match dir.name.as_str() {
            "typeAlias" => match handle_type_alias(&dir) {
                Ok(map) => type_aliases = map,
                Err(err) => violations.extend(err.violations),
            },
            _ => {
                // TODO: handle other directives
            }
        }
    }

    if !violations.is_empty() {
        return Err(GraphQLSchemaValidationError { violations });
    }

    return Ok(Scalar {
        name: def.name.clone(),
        description: def.description.clone(),
        position: build_position(&def.position),
        type_aliases,
    });
}

fn build_object_type_definition(
    def: &schema::ObjectType<String>,
) -> Result<Object, GraphQLSchemaValidationError> {
    let mut violations = vec![];
    let mut fields = vec![];

    for fdef in def.fields.iter() {
        match build_field_definition(&fdef) {
            Ok(fdef) => fields.push(fdef),
            Err(err) => violations.extend(err.violations),
        }
    }

    if !violations.is_empty() {
        return Err(GraphQLSchemaValidationError { violations });
    }

    return Ok(Object {
        name: def.name.clone(),
        description: def.description.clone(),
        interfaces: def.implements_interfaces.clone(),
        fields,
        position: build_position(&def.position),
    });
}

fn build_object_type_extension(
    def: &schema::ObjectTypeExtension<String>,
) -> Result<ObjectExtension, GraphQLSchemaValidationError> {
    let mut violations = vec![];
    let mut fields = vec![];

    for fdef in def.fields.iter() {
        match build_field_definition(&fdef) {
            Ok(fdef) => fields.push(fdef),
            Err(err) => violations.extend(err.violations),
        }
    }

    if !violations.is_empty() {
        return Err(GraphQLSchemaValidationError { violations });
    }

    return Ok(ObjectExtension {
        name: def.name.clone(),
        interfaces: def.implements_interfaces.clone(),
        fields,
        position: build_position(&def.position),
    });
}

fn build_field_definition(
    def: &schema::Field<String>,
) -> Result<Field, GraphQLSchemaValidationError> {
    let mut deprecation_reason: Option<String> = None;
    let mut resolve: Option<ResolverConfig> = None;
    let mut source_configs: Vec<SourceConfig> = Vec::new();
    let mut violations = vec![];
    let mut args = vec![];

    for dir in def.directives.iter() {
        match dir.name.as_str() {
            "deprecated" => match handle_deprecate(&&dir) {
                Ok(reason) => deprecation_reason = Some(reason),
                Err(err) => violations.extend(err.violations),
            },
            "resolve" => match handle_resolve(&dir) {
                Ok(config) => resolve = Some(config),
                Err(err) => violations.extend(err.violations),
            },
            "source" => match handle_source(&dir) {
                Ok(config) => source_configs.push(config),
                Err(err) => violations.extend(err.violations),
            },
            _ => {
                // TODO: handle other directives
            }
        }
    }

    for arg in def.arguments.iter() {
        match build_input_field_definition(&arg) {
            Ok(arg) => args.push(arg),
            Err(err) => violations.extend(err.violations),
        }
    }

    if !violations.is_empty() {
        return Err(GraphQLSchemaValidationError { violations });
    }

    return Ok(Field {
        name: def.name.clone(),
        description: def.description.clone(),
        deprecation_reason,
        args,
        field_type: build_type_expression(&def.field_type),
        resolve,
        source_configs,
        position: build_position(&def.position),
    });
}

fn build_type_expression(def: &schema::Type<String>) -> TypeExpression {
    match def {
        schema::Type::NamedType(name) => TypeExpression::NamedType(name.to_string()),
        schema::Type::ListType(inner) => {
            TypeExpression::ListType(Box::new(build_type_expression(inner)))
        }
        schema::Type::NonNullType(inner) => {
            TypeExpression::NonNullType(Box::new(build_type_expression(inner)))
        }
    }
}

fn build_interface_type_definition(
    def: &schema::InterfaceType<String>,
) -> Result<Interface, GraphQLSchemaValidationError> {
    let mut violations = vec![];
    let mut fields = vec![];

    for fdef in def.fields.iter() {
        match build_field_definition(&fdef) {
            Ok(fdef) => fields.push(fdef),
            Err(err) => violations.extend(err.violations),
        }
    }

    if !violations.is_empty() {
        return Err(GraphQLSchemaValidationError { violations });
    }

    return Ok(Interface {
        name: def.name.clone(),
        description: def.description.clone(),
        interfaces: def.implements_interfaces.clone(),
        fields,
        position: build_position(&def.position),
    });
}

fn build_interface_extension(
    def: &schema::InterfaceTypeExtension<String>,
) -> Result<InterfaceTypeExtension, GraphQLSchemaValidationError> {
    let mut violations = vec![];
    let mut fields = vec![];

    for fdef in def.fields.iter() {
        match build_field_definition(&fdef) {
            Ok(fdef) => fields.push(fdef),
            Err(err) => violations.extend(err.violations),
        }
    }

    if !violations.is_empty() {
        return Err(GraphQLSchemaValidationError { violations });
    }

    return Ok(InterfaceTypeExtension {
        name: def.name.clone(),
        interfaces: def.implements_interfaces.clone(),
        fields,
        position: build_position(&def.position),
    });
}

fn build_union_type_definition(
    def: &schema::UnionType<String>,
) -> Result<Union, GraphQLSchemaValidationError> {
    return Ok(Union {
        name: def.name.clone(),
        description: def.description.clone(),
        types: def.types.clone(),
        position: build_position(&def.position),
    });
}

fn build_union_type_extension(
    def: &schema::UnionTypeExtension<String>,
) -> Result<UnionExtension, GraphQLSchemaValidationError> {
    return Ok(UnionExtension {
        name: def.name.clone(),
        types: def.types.clone(),
        position: build_position(&def.position),
    });
}

fn build_enum_type_definition(
    def: &schema::EnumType<String>,
) -> Result<Enum, GraphQLSchemaValidationError> {
    let mut violations = vec![];
    let mut values = vec![];

    for vdef in def.values.iter() {
        match build_enum_value_definition(&vdef) {
            Ok(vdef) => values.push(vdef),
            Err(err) => violations.extend(err.violations),
        }
    }

    if !violations.is_empty() {
        return Err(GraphQLSchemaValidationError { violations });
    }

    return Ok(Enum {
        name: def.name.clone(),
        description: def.description.clone(),
        values,
        position: build_position(&def.position),
    });
}

fn build_enum_type_extension(
    def: &schema::EnumTypeExtension<String>,
) -> Result<EnumExtension, GraphQLSchemaValidationError> {
    let mut violations = vec![];
    let mut values = vec![];

    for vdef in def.values.iter() {
        match build_enum_value_definition(&vdef) {
            Ok(vdef) => values.push(vdef),
            Err(err) => violations.extend(err.violations),
        }
    }

    if !violations.is_empty() {
        return Err(GraphQLSchemaValidationError { violations });
    }

    return Ok(EnumExtension {
        name: def.name.clone(),
        values,
        position: build_position(&def.position),
    });
}

fn build_enum_value_definition(
    def: &schema::EnumValue<String>,
) -> Result<EnumValue, GraphQLSchemaValidationError> {
    let mut deprecation_reason: Option<String> = None;
    let mut violations = vec![];

    for dir in def.directives.iter() {
        match dir.name.as_str() {
            "deprecated" => match handle_deprecate(&dir) {
                Ok(reason) => deprecation_reason = Some(reason),
                Err(err) => violations.extend(err.violations),
            },
            _ => {
                // TODO: handle other directives
            }
        }
    }

    if !violations.is_empty() {
        return Err(GraphQLSchemaValidationError { violations });
    }

    return Ok(EnumValue {
        name: def.name.clone(),
        description: def.description.clone(),
        deprecation_reason,
        position: build_position(&def.position),
    });
}

fn build_input_type_definition(
    def: &schema::InputObjectType<String>,
) -> Result<Input, GraphQLSchemaValidationError> {
    let mut violations = vec![];
    let mut fields = vec![];

    for fdef in def.fields.iter() {
        match build_input_field_definition(&fdef) {
            Ok(fdef) => fields.push(fdef),
            Err(err) => violations.extend(err.violations),
        }
    }

    if !violations.is_empty() {
        return Err(GraphQLSchemaValidationError { violations });
    }

    return Ok(Input {
        name: def.name.clone(),
        description: def.description.clone(),
        fields,
        position: build_position(&def.position),
    });
}

fn build_input_type_extension(
    def: &schema::InputObjectTypeExtension<String>,
) -> Result<InputExtension, GraphQLSchemaValidationError> {
    let mut violations = vec![];
    let mut fields = vec![];

    for fdef in def.fields.iter() {
        match build_input_field_definition(&fdef) {
            Ok(fdef) => fields.push(fdef),
            Err(err) => violations.extend(err.violations),
        }
    }

    if !violations.is_empty() {
        return Err(GraphQLSchemaValidationError { violations });
    }

    return Ok(InputExtension {
        name: def.name.clone(),
        fields,
        position: build_position(&def.position),
    });
}

fn build_input_field_definition(
    def: &schema::InputValue<String>,
) -> Result<InputValue, GraphQLSchemaValidationError> {
    let mut deprecation_reason: Option<String> = None;
    let mut violations = vec![];

    for dir in def.directives.iter() {
        match dir.name.as_str() {
            "deprecated" => match handle_deprecate(&dir) {
                Ok(reason) => deprecation_reason = Some(reason),
                Err(err) => violations.extend(err.violations),
            },
            _ => {
                // TODO: handle other directives
            }
        }
    }

    if !violations.is_empty() {
        return Err(GraphQLSchemaValidationError { violations });
    }

    return Ok(InputValue {
        name: def.name.clone(),
        description: def.description.clone(),
        field_type: build_type_expression(&def.value_type),
        default_value: def.default_value.clone().map(build_value),
        position: build_position(&def.position),
        deprecation_reason,
    });
}

fn build_value(def: schema::Value<String>) -> Value {
    match def {
        schema::Value::Variable(_) => unreachable!(),
        schema::Value::Int(v) => Value::Int(v.as_i64().unwrap().try_into().unwrap()),
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
    directive: &schema::Directive<String>,
) -> Result<String, GraphQLSchemaValidationError> {
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
                        pos: build_position(&directive.position),
                        name: key.clone(),
                        value: value.to_string(),
                    });
                }
            },
            _ => {
                violations.push(GraphQLValidationViolation::KnownArgumentNames {
                    pos: build_position(&directive.position),
                    name: key.clone(),
                });
            }
        }
    }

    if !violations.is_empty() {
        return Err(GraphQLSchemaValidationError { violations });
    }

    return Ok(reason);
}

fn handle_resolve(
    directive: &schema::Directive<String>,
) -> Result<ResolverConfig, GraphQLSchemaValidationError> {
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
                        pos: build_position(&directive.position),
                        name: key.clone(),
                        value: value.to_string(),
                    });
                }
            },
            _ => {
                violations.push(GraphQLValidationViolation::KnownArgumentNames {
                    pos: build_position(&directive.position),
                    name: key.clone(),
                });
            }
        }
    }

    if !violations.is_empty() {
        return Err(GraphQLSchemaValidationError { violations });
    }

    return Ok(def);
}

fn handle_source(
    directive: &schema::Directive<String>,
) -> Result<SourceConfig, GraphQLSchemaValidationError> {
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
                        pos: build_position(&directive.position),
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
                        pos: build_position(&directive.position),
                        name: key.clone(),
                        value: value.to_string(),
                    });
                }
            },
            _ => {
                violations.push(GraphQLValidationViolation::KnownArgumentNames {
                    pos: build_position(&directive.position),
                    name: key.clone(),
                });
            }
        }
    }

    if name.is_none() {
        violations.push(GraphQLValidationViolation::ProvidedRequiredArguments {
            pos: build_position(&directive.position),
            name: "name".to_string(),
        });
    }
    if type_.is_none() {
        violations.push(GraphQLValidationViolation::ProvidedRequiredArguments {
            pos: build_position(&directive.position),
            name: "type".to_string(),
        });
    }

    if !violations.is_empty() {
        return Err(GraphQLSchemaValidationError { violations });
    }

    let def = SourceConfig {
        name: name.unwrap(),
        type_: type_.unwrap(),
    };

    return Ok(def);
}

fn handle_source_type(
    directive: &schema::Directive<String>,
) -> Result<HashMap<String, String>, GraphQLSchemaValidationError> {
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
                    pos: build_position(&directive.position),
                    name: key.clone(),
                    value: value.to_string(),
                });
            }
        }
    }
    if !violations.is_empty() {
        return Err(GraphQLSchemaValidationError { violations });
    }

    return Ok(map);
}

fn handle_type_alias(
    directive: &schema::Directive<String>,
) -> Result<HashMap<String, String>, GraphQLSchemaValidationError> {
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
                    pos: build_position(&directive.position),
                    name: key.clone(),
                    value: value.to_string(),
                });
            }
        }
    }
    if !violations.is_empty() {
        return Err(GraphQLSchemaValidationError { violations });
    }

    return Ok(map);
}

fn build_position(def: &graphql_parser::Pos) -> Position {
    return Position {
        line: def.line,
        column: def.column,
    };
}

#[derive(Debug)]
pub enum SchemaBuildingError {
    GraphQLParserError(graphql_parser::schema::ParseError),
    GraphQLSchemaValidationError(GraphQLSchemaValidationError),
    IOError(std::io::Error),
}

impl fmt::Display for SchemaBuildingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SchemaBuildingError::GraphQLParserError(err) => {
                write!(f, "GraphQL Parser Error: {}", err)
            }
            SchemaBuildingError::GraphQLSchemaValidationError(err) => {
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
struct GraphQLSchemaValidationError {
    violations: Vec<GraphQLValidationViolation>,
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
