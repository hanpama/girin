use serde::Serialize;
use std::collections::HashMap;

pub struct Schema {
    pub root_module: Module,
}

impl Schema {
    pub fn get_query(&self) -> Option<Box<ObjectTypeDefinition>> {
        None // Return None if "Query" definition is not found or not of ObjectTypeDefinition type
    }
}

#[derive(Serialize, Debug)]
pub struct Module {
    pub path: String,
    pub name: String,
    pub children: Vec<Module>,
    pub submodules: Vec<Submodule>,
}

#[derive(Serialize, Debug)]
pub struct Submodule {
    pub path: String,
    pub name: String,
    pub definitions: Vec<Definition>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum Definition {
    SchemaDefinition(SchemaDefinition),
    ScalarTypeDefinition(ScalarTypeDefinition),
    ObjectTypeDefinition(ObjectTypeDefinition),
    InterfaceTypeDefinition(InterfaceTypeDefinition),
    UnionTypeDefinition(UnionTypeDefinition),
    EnumTypeDefinition(EnumTypeDefinition),
    InputObjectTypeDefinition(InputObjectTypeDefinition),
    // ScalarTypeExtension(ScalarTypeExtension),
    ObjectTypeExtension(ObjectTypeExtension),
    InterfaceTypeExtension(InterfaceTypeExtension),
    UnionTypeExtension(UnionTypeExtension),
    EnumTypeExtension(EnumTypeExtension),
    InputObjectTypeExtension(InputObjectTypeExtension),
}

#[derive(Serialize, Debug)]
pub struct SchemaDefinition {
    pub query: Option<String>,
    pub mutation: Option<String>,
    pub subscription: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct ScalarTypeDefinition {
    pub name: String,
    pub description: Option<String>,
    pub position: Position,
    pub type_aliases: HashMap<String, String>,
}

#[derive(Serialize, Debug)]
pub struct ObjectTypeDefinition {
    pub fields: Vec<FieldDefinition>,
    pub name: String,
    pub description: Option<String>,
    pub interfaces: Vec<String>,
    pub position: Position,
}

#[derive(Serialize, Debug)]
pub struct InterfaceTypeDefinition {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<FieldDefinition>,
    pub interfaces: Vec<String>,
    pub position: Position,
}

#[derive(Serialize, Debug)]
pub struct InterfaceTypeExtension {
    pub name: String,
    pub fields: Vec<FieldDefinition>,
    pub interfaces: Vec<String>,
    pub position: Position,
}

#[derive(Serialize, Debug)]
pub struct InputObjectTypeDefinition {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<InputFieldDefinition>,
    pub position: Position,
}

#[derive(Serialize, Debug)]
pub struct UnionTypeDefinition {
    pub name: String,
    pub description: Option<String>,
    pub types: Vec<String>,
    pub position: Position,
}

#[derive(Serialize, Debug)]
pub struct UnionTypeExtension {
    pub name: String,
    pub types: Vec<String>,
    pub position: Position,
}

#[derive(Serialize, Debug)]
pub struct EnumTypeDefinition {
    pub name: String,
    pub description: Option<String>,
    pub values: Vec<EnumValueDefinition>,
    pub position: Position,
}

#[derive(Serialize, Debug)]
pub struct FieldDefinition {
    pub name: String,
    pub description: Option<String>,
    pub deprecation_reason: Option<String>,
    pub args: Vec<InputFieldDefinition>,
    pub field_type: TypeExpression,
    pub resolve: Option<ResolverConfig>,
    pub source_configs: Vec<SourceConfig>,
    pub position: Position,
}

#[derive(Serialize, Debug)]
pub struct ResolverConfig {
    pub sync: bool,
}

#[derive(Serialize, Debug)]
pub struct SourceConfig {
    pub name: String,
    pub type_: TypeExpression,
}

#[derive(Serialize, Debug)]
pub struct InputFieldDefinition {
    pub name: String,
    pub description: Option<String>,
    pub deprecation_reason: Option<String>,
    pub field_type: TypeExpression,
    pub default_value: Option<Value>,
    pub position: Position,
}

#[derive(Serialize, Debug)]
pub struct EnumValueDefinition {
    pub name: String,
    pub description: Option<String>,
    pub deprecation_reason: Option<String>,
    pub position: Position,
}

#[derive(Serialize, Debug)]
pub struct ObjectTypeExtension {
    pub name: String,
    pub interfaces: Vec<String>,
    pub fields: Vec<FieldDefinition>,
    pub position: Position,
}

#[derive(Serialize, Debug)]
pub struct InputObjectTypeExtension {
    pub name: String,
    pub fields: Vec<InputFieldDefinition>,
    pub position: Position,
}

#[derive(Serialize, Debug)]
pub struct EnumTypeExtension {
    pub name: String,
    pub values: Vec<EnumValueDefinition>,
    pub position: Position,
}

#[derive(Serialize, Debug)]
pub struct Position {
    #[serde(skip)]
    pub file: String,
    /// One-based line number
    pub line: usize,
    /// One-based column number
    pub column: usize,
}

#[derive(Serialize, Debug)]
pub enum TypeExpression {
    NamedType(String),
    ListType(Box<TypeExpression>),
    NonNullType(Box<TypeExpression>),
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Enum(String),
    Null,
    List(Vec<Value>),
    Object(HashMap<String, Value>),
}
