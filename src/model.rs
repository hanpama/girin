pub struct Project {
    pub root_module: Module,
}

impl Project {
    pub fn get_query(&self) -> Option<Box<ObjectTypeDefinition>> {
        None // Return None if "Query" definition is not found or not of ObjectTypeDefinition type
    }
}

pub struct Module {
    pub path: String,
    pub name: String,
    pub children: Vec<Module>,
    pub submodules: Vec<Submodule>,
}

pub struct Submodule {
    pub path: String,
    pub name: String,
    pub definitions: Vec<Definition>,
}

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
    // InterfaceTypeExtension(InterfaceTypeExtension),
    // UnionTypeExtension(UnionTypeExtension),
    EnumTypeExtension(EnumTypeExtension),
    InputObjectTypeExtension(InputObjectTypeExtension),
}

pub struct SchemaDefinition {
    pub query: Option<String>,
    pub mutation: Option<String>,
    pub subscription: Option<String>,
}

pub struct ScalarTypeDefinition {
    pub name: String,
    pub description: Option<String>,
    pub position: Position,
}

pub struct ObjectTypeDefinition {
    pub fields: Vec<FieldDefinition>,
    pub name: String,
    pub description: Option<String>,
    pub interfaces: Vec<String>,
    pub position: Position,
}

pub struct InterfaceTypeDefinition {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<FieldDefinition>,
    pub position: Position,
}

pub struct InputObjectTypeDefinition {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<InputFieldDefinition>,
    pub position: Position,
}

pub struct UnionTypeDefinition {
    pub name: String,
    pub description: Option<String>,
    pub types: Vec<String>,
    pub position: Position,
}

pub struct EnumTypeDefinition {
    pub name: String,
    pub description: Option<String>,
    pub values: Vec<EnumValueDefinition>,
    pub position: Position,
}

pub struct FieldDefinition {
    pub name: String,
    pub description: Option<String>,
    pub deprecation_reason: Option<String>,
    pub args: Vec<InputFieldDefinition>,
    pub field_type: TypeExpression,
    pub resolve: Option<ResolverDefinition>,
    pub position: Position,
}

pub struct ResolverDefinition {
    pub sync: bool,
}

pub struct InputFieldDefinition {
    pub name: String,
    pub description: Option<String>,
    pub deprecation_reason: String,
    pub field_type: TypeExpression,
    pub default_value: String, // TODO!
    pub position: Position,
}

pub struct EnumValueDefinition {
    pub name: String,
    pub description: Option<String>,
    pub deprecation_reason: String,
    pub position: Position,
}

pub struct ObjectTypeExtension {
    pub name: String,
    pub interfaces: Vec<String>,
    pub fields: Vec<FieldDefinition>,
    pub position: Position,
}

pub struct InputObjectTypeExtension {
    pub name: String,
    pub fields: Vec<InputFieldDefinition>,
    pub position: Position,
}

pub struct EnumTypeExtension {
    pub name: String,
    pub values: Vec<EnumValueDefinition>,
    pub position: Position,
}

pub struct Position {
    pub file: String,
    /// One-based line number
    pub line: usize,
    /// One-based column number
    pub column: usize,
}

pub enum TypeExpression {
    NamedType(String),
    ListType(Box<TypeExpression>),
    NonNullType(Box<TypeExpression>),
}
