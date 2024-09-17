use super::{violation::GraphQLValidationViolation, ScalarDefinition};
use crate::schema::{
    Definition, DirectiveDefinition, EnumDefinition, EnumExtension, EnumValue, Field,
    InputDefinition, InputExtension, InputValue, InterfaceDefinition, InterfaceExtension,
    ObjectDefinition, ObjectExtension, Position, ResolveConfig, SchemaDefinition, SourceConfig,
    TypeExpression, UnionDefinition, UnionExtension, Value,
};
use graphql_parser::schema::{self, TypeDefinition, TypeExtension};
use std::collections::{BTreeMap, HashMap};
use std::path::Path;

pub fn construct_definitions(
    file: &Path,
    document: schema::Document<String>,
) -> Result<Vec<Definition>> {
    let mut definitions = vec![];
    let mut violations = vec![];

    for node in document.definitions {
        match node {
            schema::Definition::SchemaDefinition(i) => match construct_schema_definition(file, i) {
                Ok(def) => definitions.push(def),
                Err(err) => violations.extend(err.violations),
            },
            schema::Definition::TypeDefinition(i) => match construct_type_definition(file, i) {
                Ok(def) => definitions.push(def),
                Err(err) => violations.extend(err.violations),
            },
            schema::Definition::TypeExtension(i) => match construct_type_extension(file, i) {
                Ok(def) => definitions.push(def),
                Err(err) => violations.extend(err.violations),
            },
            schema::Definition::DirectiveDefinition(_) => {
                todo!()
            }
        }
    }

    if !violations.is_empty() {
        return Err(Error { violations });
    }

    return Ok(definitions);
}

fn construct_schema_definition(
    file: &Path,
    def: schema::SchemaDefinition<String>,
) -> Result<Definition> {
    return Ok(Definition::SchemaDefinition(SchemaDefinition {
        query: def.query.clone(),
        mutation: def.mutation.clone(),
        subscription: def.subscription.clone(),
        position: build_position(file, &def.position),
    }));
}

fn construct_type_definition(file: &Path, def: TypeDefinition<String>) -> Result<Definition> {
    Ok(match def {
        TypeDefinition::Object(def) => {
            Definition::ObjectDefinition(construct_object_definition(file, &def)?)
        }
        TypeDefinition::Interface(def) => {
            Definition::InterfaceDefinition(construct_interface_definition(file, &def)?)
        }
        TypeDefinition::Union(def) => {
            Definition::UnionDefinition(build_union_type_definition(file, &def)?)
        }
        TypeDefinition::Enum(def) => {
            Definition::EnumDefinition(build_enum_type_definition(file, &def)?)
        }
        TypeDefinition::InputObject(def) => {
            Definition::InputDefinition(build_input_type_definition(file, &def)?)
        }
        TypeDefinition::Scalar(def) => {
            Definition::ScalarDefinition(build_scalar_type_definition(file, &def)?)
        }
    })
}

fn construct_type_extension(module: &Path, def: TypeExtension<String>) -> Result<Definition> {
    Ok(match def {
        TypeExtension::Object(def) => {
            Definition::ObjectExtension(construct_object_extension(module, &def)?)
        }
        TypeExtension::Interface(def) => {
            Definition::InterfaceExtension(construct_interface_extension(module, &def)?)
        }
        TypeExtension::Union(def) => {
            Definition::UnionExtension(build_union_type_extension(module, &def)?)
        }
        TypeExtension::Enum(def) => {
            Definition::EnumExtension(build_enum_type_extension(module, &def)?)
        }
        TypeExtension::InputObject(def) => {
            Definition::InputExtension(build_input_type_extension(module, &def)?)
        }
        TypeExtension::Scalar(def) => {
            todo!()
        }
    })
}

fn construct_object_definition(
    file: &Path,
    def: &schema::ObjectType<String>,
) -> Result<ObjectDefinition> {
    let mut violations = vec![];
    let mut fields = vec![];

    for fdef in def.fields.iter() {
        match construct_field_definition(file, &def.name, &fdef) {
            Ok(fdef) => fields.push(fdef),
            Err(err) => violations.extend(err.violations),
        }
    }

    if !violations.is_empty() {
        return Err(Error { violations });
    }

    return Ok(ObjectDefinition {
        name: def.name.clone(),
        description: def.description.clone(),
        interfaces: def.implements_interfaces.clone(),
        fields,
        position: build_position(file, &def.position),
    });
}

fn construct_object_extension(
    file: &Path,
    def: &schema::ObjectTypeExtension<String>,
) -> Result<ObjectExtension> {
    let mut violations = vec![];
    let mut fields = vec![];

    for fdef in def.fields.iter() {
        match construct_field_definition(file, &def.name, &fdef) {
            Ok(fdef) => fields.push(fdef),
            Err(err) => violations.extend(err.violations),
        }
    }

    if !violations.is_empty() {
        return Err(Error { violations });
    }

    return Ok(ObjectExtension {
        name: def.name.clone(),
        interfaces: def.implements_interfaces.clone(),
        fields,
        position: build_position(file, &def.position),
    });
}

fn construct_interface_definition(
    file: &Path,
    def: &schema::InterfaceType<String>,
) -> Result<InterfaceDefinition> {
    let mut violations = vec![];
    let mut fields = vec![];

    for fdef in def.fields.iter() {
        match construct_field_definition(file, &def.name, &fdef) {
            Ok(fdef) => fields.push(fdef),
            Err(err) => violations.extend(err.violations),
        }
    }

    if !violations.is_empty() {
        return Err(Error { violations });
    }

    return Ok(InterfaceDefinition {
        name: def.name.clone(),
        description: def.description.clone(),
        interfaces: def.implements_interfaces.clone(),
        fields,
        position: build_position(file, &def.position),
    });
}

fn construct_interface_extension(
    file: &Path,
    def: &schema::InterfaceTypeExtension<String>,
) -> Result<InterfaceExtension> {
    let mut violations = vec![];
    let mut fields = vec![];

    for fdef in def.fields.iter() {
        match construct_field_definition(file, &def.name, &fdef) {
            Ok(fdef) => fields.push(fdef),
            Err(err) => violations.extend(err.violations),
        }
    }

    if !violations.is_empty() {
        return Err(Error { violations });
    }

    return Ok(InterfaceExtension {
        name: def.name.clone(),
        interfaces: def.implements_interfaces.clone(),
        fields,
        position: build_position(file, &def.position),
    });
}

pub fn construct_field_definition(
    file: &Path,
    type_name: &str,
    def: &schema::Field<String>,
) -> Result<Field> {
    let mut deprecation_reason: Option<String> = None;
    let mut resolve: Option<ResolveConfig> = None;
    let mut source_configs: Vec<SourceConfig> = Vec::new();
    let mut violations = vec![];
    let mut args = vec![];

    for dir in def.directives.iter() {
        match dir.name.as_str() {
            "deprecated" => match handle_deprecate(file, &dir) {
                Ok(reason) => deprecation_reason = Some(reason),
                Err(err) => violations.extend(err.violations),
            },
            "resolve" => match handle_resolve(file, &dir) {
                Ok(config) => resolve = Some(config),
                Err(err) => violations.extend(err.violations),
            },
            "source" => match handle_source(file, &dir) {
                Ok(config) => source_configs.push(config),
                Err(err) => violations.extend(err.violations),
            },
            _ => {
                // TODO: handle other directives
            }
        }
    }

    for arg in def.arguments.iter() {
        match build_input_field_definition(file, &arg) {
            Ok(arg) => args.push(arg),
            Err(err) => violations.extend(err.violations),
        }
    }

    if !violations.is_empty() {
        return Err(Error { violations });
    }

    return Ok(Field {
        name: def.name.clone(),
        description: def.description.clone(),
        deprecation_reason,
        args,
        field_type: build_type_expression(&def.field_type),
        resolve_config: resolve,
        source_configs,
        type_name: type_name.to_string(),
        position: build_position(file, &def.position),
    });
}

fn construct_directive_definition(
    file: &Path,
    node: &schema::DirectiveDefinition<String>,
) -> Result<DirectiveDefinition> {
    // position
    // description
    // name
    // arguments
    // repeatable
    // locations
    todo!()
}

fn handle_resolve(file: &Path, directive: &schema::Directive<String>) -> Result<ResolveConfig> {
    let mut def = ResolveConfig { sync: false };
    let mut violations = vec![];

    for (key, value) in directive.arguments.iter() {
        match key.as_str() {
            "sync" => match value {
                schema::Value::Boolean(value) => {
                    def.sync = value.clone();
                }
                _ => {
                    violations.push(GraphQLValidationViolation::ValuesOfCorrectType {
                        pos: build_position(file, &directive.position),
                        name: key.clone(),
                        value: value.to_string(),
                    });
                }
            },
            _ => {
                violations.push(GraphQLValidationViolation::KnownArgumentNames {
                    pos: build_position(file, &directive.position),
                    name: key.clone(),
                });
            }
        }
    }

    if !violations.is_empty() {
        return Err(Error { violations });
    }

    return Ok(def);
}

fn handle_source(file: &Path, directive: &schema::Directive<String>) -> Result<SourceConfig> {
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
                        pos: build_position(file, &directive.position),
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
                        pos: build_position(file, &directive.position),
                        name: key.clone(),
                        value: value.to_string(),
                    });
                }
            },
            _ => {
                violations.push(GraphQLValidationViolation::KnownArgumentNames {
                    pos: build_position(file, &directive.position),
                    name: key.clone(),
                });
            }
        }
    }

    if name.is_none() {
        violations.push(GraphQLValidationViolation::ProvidedRequiredArguments {
            pos: build_position(file, &directive.position),
            name: "name".to_string(),
        });
    }
    if type_.is_none() {
        type_ = Some(TypeExpression::NamedType("String".to_string()));
    }

    if !violations.is_empty() {
        return Err(Error { violations });
    }

    let def = SourceConfig {
        name: name.unwrap(),
        type_: type_.unwrap(),
    };

    return Ok(def);
}

fn build_union_type_definition(
    file: &Path,
    def: &schema::UnionType<String>,
) -> Result<UnionDefinition> {
    return Ok(UnionDefinition {
        name: def.name.clone(),
        description: def.description.clone(),
        types: def.types.clone(),

        position: build_position(file, &def.position),
    });
}

fn build_union_type_extension(
    file: &Path,
    def: &schema::UnionTypeExtension<String>,
) -> Result<UnionExtension> {
    return Ok(UnionExtension {
        name: def.name.clone(),
        types: def.types.clone(),

        position: build_position(file, &def.position),
    });
}

fn build_enum_type_definition(
    file: &Path,
    def: &schema::EnumType<String>,
) -> Result<EnumDefinition> {
    let mut violations = vec![];
    let mut values = vec![];

    for vdef in def.values.iter() {
        match build_enum_value_definition(file, &vdef) {
            Ok(vdef) => values.push(vdef),
            Err(err) => violations.extend(err.violations),
        }
    }

    if !violations.is_empty() {
        return Err(Error { violations });
    }

    return Ok(EnumDefinition {
        name: def.name.clone(),
        description: def.description.clone(),
        values,

        position: build_position(file, &def.position),
    });
}

fn build_enum_type_extension(
    file: &Path,
    def: &schema::EnumTypeExtension<String>,
) -> Result<EnumExtension> {
    let mut violations = vec![];
    let mut values = vec![];

    for vdef in def.values.iter() {
        match build_enum_value_definition(file, &vdef) {
            Ok(vdef) => values.push(vdef),
            Err(err) => violations.extend(err.violations),
        }
    }

    if !violations.is_empty() {
        return Err(Error { violations });
    }

    return Ok(EnumExtension {
        name: def.name.clone(),
        values,

        position: build_position(file, &def.position),
    });
}

fn build_enum_value_definition(file: &Path, def: &schema::EnumValue<String>) -> Result<EnumValue> {
    let mut deprecation_reason: Option<String> = None;
    let mut violations = vec![];

    for dir in def.directives.iter() {
        match dir.name.as_str() {
            "deprecated" => match handle_deprecate(file, &dir) {
                Ok(reason) => deprecation_reason = Some(reason),
                Err(err) => violations.extend(err.violations),
            },
            _ => {
                // TODO: handle other directives
            }
        }
    }

    if !violations.is_empty() {
        return Err(Error { violations });
    }

    return Ok(EnumValue {
        name: def.name.clone(),
        description: def.description.clone(),
        deprecation_reason,
        position: build_position(file, &def.position),
    });
}

fn build_input_type_definition(
    file: &Path,
    def: &schema::InputObjectType<String>,
) -> Result<InputDefinition> {
    let mut violations = vec![];
    let mut fields = vec![];

    for fdef in def.fields.iter() {
        match build_input_field_definition(file, &fdef) {
            Ok(fdef) => fields.push(fdef),
            Err(err) => violations.extend(err.violations),
        }
    }

    if !violations.is_empty() {
        return Err(Error { violations });
    }

    return Ok(InputDefinition {
        name: def.name.clone(),
        description: def.description.clone(),
        fields,

        position: build_position(file, &def.position),
    });
}

fn build_input_type_extension(
    file: &Path,
    def: &schema::InputObjectTypeExtension<String>,
) -> Result<InputExtension> {
    let mut violations = vec![];
    let mut fields = vec![];

    for fdef in def.fields.iter() {
        match build_input_field_definition(file, &fdef) {
            Ok(fdef) => fields.push(fdef),
            Err(err) => violations.extend(err.violations),
        }
    }

    if !violations.is_empty() {
        return Err(Error { violations });
    }

    return Ok(InputExtension {
        name: def.name.clone(),
        fields,

        position: build_position(file, &def.position),
    });
}

fn build_input_field_definition(
    file: &Path,
    def: &schema::InputValue<String>,
) -> Result<InputValue> {
    let mut deprecation_reason: Option<String> = None;
    let mut violations = vec![];

    for dir in def.directives.iter() {
        match dir.name.as_str() {
            "deprecated" => match handle_deprecate(file, &dir) {
                Ok(reason) => deprecation_reason = Some(reason),
                Err(err) => violations.extend(err.violations),
            },
            _ => {
                // TODO: handle other directives
            }
        }
    }

    if !violations.is_empty() {
        return Err(Error { violations });
    }

    return Ok(InputValue {
        name: def.name.clone(),
        description: def.description.clone(),
        field_type: build_type_expression(&def.value_type),
        default_value: def.default_value.clone().map(build_value),
        position: build_position(file, &def.position),
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
                .collect::<BTreeMap<String, Value>>(),
        ),
    }
}

fn handle_deprecate(file: &Path, directive: &schema::Directive<String>) -> Result<String> {
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
                        pos: build_position(file, &directive.position),
                        name: key.clone(),
                        value: value.to_string(),
                    });
                }
            },
            _ => {
                violations.push(GraphQLValidationViolation::KnownArgumentNames {
                    pos: build_position(file, &directive.position),
                    name: key.clone(),
                });
            }
        }
    }

    if !violations.is_empty() {
        return Err(Error { violations });
    }

    return Ok(reason);
}

pub fn build_scalar_type_definition(
    file: &Path,
    def: &schema::ScalarType<String>,
) -> Result<ScalarDefinition> {
    let mut violations = vec![];
    let mut type_aliases = HashMap::<String, String>::new();

    for dir in def.directives.iter() {
        match dir.name.as_str() {
            "type" => match handle_type_alias(file, &dir) {
                Ok(map) => type_aliases = map,
                Err(err) => violations.extend(err.violations),
            },
            _ => {
                // TODO: handle other directives
            }
        }
    }

    if !violations.is_empty() {
        return Err(Error { violations });
    }

    return Ok(ScalarDefinition {
        name: def.name.clone(),
        description: def.description.clone(),
        position: build_position(file, &def.position),
        type_aliases,
    });
}

fn handle_type_alias(
    file: &Path,
    directive: &schema::Directive<String>,
) -> Result<HashMap<String, String>> {
    let mut map = HashMap::new();
    let mut violations = vec![];

    for (key, value) in directive.arguments.iter() {
        match key.as_str() {
            "alias" => match value {
                schema::Value::String(value) => {
                    let segments: Vec<&str> = value.splitn(2, ":").collect();
                    if segments.len() != 2 {
                        violations.push(GraphQLValidationViolation::ValuesOfCorrectType {
                            pos: build_position(file, &directive.position),
                            name: key.clone(),
                            value: value.to_string(),
                        });
                        continue;
                    }
                    map.insert(segments[0].to_owned(), segments[1].to_owned());
                }
                _ => {
                    violations.push(GraphQLValidationViolation::ValuesOfCorrectType {
                        pos: build_position(file, &directive.position),
                        name: key.clone(),
                        value: value.to_string(),
                    });
                }
            },
            _ => {
                violations.push(GraphQLValidationViolation::KnownArgumentNames {
                    pos: build_position(file, &directive.position),
                    name: key.clone(),
                });
            }
        }
    }

    if !violations.is_empty() {
        return Err(Error { violations });
    }

    return Ok(map);
}

fn build_position(file: &Path, def: &graphql_parser::Pos) -> Position {
    return Position {
        file: file.to_path_buf(),
        line: def.line,
        column: def.column,
    };
}

fn format_module(root: &Path, file: &Path) -> Vec<String> {
    file.strip_prefix(root)
        .unwrap()
        .with_extension("")
        .components()
        .map(|p| p.as_os_str().to_str().unwrap().to_owned())
        .collect()
}

pub fn build_type_expression(def: &schema::Type<String>) -> TypeExpression {
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

// result
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub violations: Vec<GraphQLValidationViolation>,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for violation in self.violations.iter() {
            write!(f, "{}\n", violation)?;
        }
        Ok(())
    }
}
