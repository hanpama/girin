use std::collections::HashMap;

pub struct Schema {
    // pub root_module: Module,
    pub query: Option<String>,
    pub mutation: Option<String>,
    pub subscription: Option<String>,
    pub children: Vec<Module>,
    pub submodules: Vec<Submodule>,
    definition_locations: HashMap<String, Location>,
    extension_locations: HashMap<String, Vec<Location>>,
}

impl Schema {
    pub fn new() -> Self {
        Self {
            query: None,
            mutation: None,
            subscription: None,
            children: Vec::new(),
            submodules: Vec::new(),
            definition_locations: HashMap::new(),
            extension_locations: HashMap::new(),
        }
    }

    // pub fn get_query(&self) -> Option<&ObjectTypeDefinition> {
    //     None // Return None if "Query" definition is not found or not of ObjectTypeDefinition type
    // }

    pub fn get_definition(&self, name: &str) -> Option<&Definition> {
        None // Return None if definition is not found
    }

    // 스키마 객체에 주요한 메서드들이 구현됨
    // get definition
    // collect extensions 등

    // local해 보이지만 실제로는 global한 기능들:
    // - object type의 fields 전체 수집
    // - interface type의 필드 수집
    // - union의 types 수집
    // - enum의 values 수집
    // - interface type의 possible types 수집

    // 또 특정 모듈의 ancestor path 구하는 거 필요하고

    pub fn collect_object_source_configs(&self, def: &Object) -> Vec<SourceConfig> {
        let mut cfs = def.collect_source_configs();
        for ext in self.find_object_extensions(def) {
            // cfs.iter().chain()\
            ext.collect_source_configs();
            cfs.extend(ext.collect_source_configs());
        }
        cfs
    }

    pub fn collect_object_fields<'a>(&'a self, def: &'a Object) -> Vec<&'a FieldDefinition> {
        def.fields.iter().collect()
    }

    pub fn collect_interface_fields<'a>(&'a self, def: &'a Interface) -> Vec<&'a FieldDefinition> {
        def.fields.iter().collect()
    }

    pub fn collect_interface_source_configs(&self, def: &Interface) -> Vec<SourceConfig> {
        def.collect_source_configs()
    }

    pub fn collect_input_fields<'a>(&'a self, def: &'a Input) -> Vec<&'a InputFieldDefinition> {
        def.fields.iter().collect()
    }

    pub fn collect_input_source_configs(&self, def: &Input) -> Vec<SourceConfig> {
        def.collect_source_configs()
    }

    pub fn collect_enum_values(&self, def: &Enum) -> Vec<String> {
        def.values.iter().map(|v| v.name.clone()).collect()
    }
    pub fn add_definition(&mut self, smd: Location, def: Definition) {
        // self.definition_locations.insert(loc.name, loc);
    }

    fn find_definition(&self, loc: Location) {
        // self.children
    }
    fn find_object_extensions(&self, def: &Object) -> Vec<&ObjectExtension> {
        // self.submodules
    }
}

#[derive(Clone)]
struct Location {
    pub path: Vec<String>,
    pub name: String,
    pub index: usize,
}

#[derive(Debug)]
pub struct Module {
    pub path: String,
    pub name: String,
    pub children: Vec<Module>,
    pub submodules: Vec<Submodule>,
}

impl Module {
    pub fn is_empty(&self) -> bool {
        self.children.is_empty() && self.submodules.is_empty()
    }
}

#[derive(Debug)]
pub struct Submodule {
    pub path: String,
    pub name: String,
    definitions: Vec<Definition>,
}

impl Submodule {
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }
}

#[derive(Debug)]
pub enum Definition {
    Scalar(Scalar),
    Object(Object),
    Interface(Interface),
    Union(Union),
    Enum(Enum),
    Input(Input),
    // ScalarTypeExtension(ScalarTypeExtension),
    ObjectExtension(ObjectExtension),
    InterfaceExtension(InterfaceTypeExtension),
    UnionExtension(UnionExtension),
    EnumExtension(EnumExtension),
    InputExtension(InputExtension),
}

// #[derive(Debug)]
// pub struct SchemaDefinition {
//     pub query: Option<String>,
//     pub mutation: Option<String>,
//     pub subscription: Option<String>,
// }

#[derive(Debug)]
pub struct Scalar {
    pub name: String,
    pub description: Option<String>,
    pub position: Position,
    pub type_aliases: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Object {
    pub fields: Vec<FieldDefinition>,
    pub name: String,
    pub description: Option<String>,
    pub interfaces: Vec<String>,
    pub position: Position,
}

impl Object {
    pub fn collect_source_configs(&self) -> Vec<SourceConfig> {
        self.fields
            .iter()
            .flat_map(|field| field.collect_source_configs())
            .collect()
    }
}

#[derive(Debug)]
pub struct Interface {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<FieldDefinition>,
    pub interfaces: Vec<String>,
    pub position: Position,
}

impl Interface {
    pub fn collect_source_configs(&self) -> Vec<SourceConfig> {
        self.fields
            .iter()
            .flat_map(|field| field.collect_source_configs())
            .collect()
    }
}

#[derive(Debug)]
pub struct InterfaceTypeExtension {
    pub name: String,
    pub fields: Vec<FieldDefinition>,
    pub interfaces: Vec<String>,
    pub position: Position,
}

#[derive(Debug)]
pub struct Input {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<InputFieldDefinition>,
    pub position: Position,
}

impl Input {
    pub fn collect_source_configs(&self) -> Vec<SourceConfig> {
        self.fields
            .iter()
            .map(|field| SourceConfig {
                name: field.name.clone(),
                type_: field.field_type.clone(),
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct Union {
    pub name: String,
    pub description: Option<String>,
    pub types: Vec<String>,
    pub position: Position,
}

#[derive(Debug)]
pub struct UnionExtension {
    pub name: String,
    pub types: Vec<String>,
    pub position: Position,
}

#[derive(Debug)]
pub struct Enum {
    pub name: String,
    pub description: Option<String>,
    pub values: Vec<EnumValueDefinition>,
    pub position: Position,
}

#[derive(Debug)]
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

impl FieldDefinition {
    pub fn collect_source_configs(&self) -> Vec<SourceConfig> {
        if !self.source_configs.is_empty() {
            return self.source_configs.clone();
        } else if self.resolve.is_some() {
            return Vec::new();
        } else if !self.args.is_empty() {
            return Vec::new();
        } else {
            vec![SourceConfig {
                name: self.name.clone(),
                type_: self.field_type.clone(),
            }]
        }
    }
}

#[derive(Debug)]
pub struct ResolverConfig {
    pub sync: bool,
}

#[derive(Debug, Clone)]
pub struct SourceConfig {
    pub name: String,
    pub type_: TypeExpression,
}

#[derive(Debug)]
pub struct InputFieldDefinition {
    pub name: String,
    pub description: Option<String>,
    pub deprecation_reason: Option<String>,
    pub field_type: TypeExpression,
    pub default_value: Option<Value>,
    pub position: Position,
}

#[derive(Debug)]
pub struct EnumValueDefinition {
    pub name: String,
    pub description: Option<String>,
    pub deprecation_reason: Option<String>,
    pub position: Position,
}

#[derive(Debug)]
pub struct ObjectExtension {
    pub name: String,
    pub interfaces: Vec<String>,
    pub fields: Vec<FieldDefinition>,
    pub position: Position,
}

#[derive(Debug)]
pub struct InputExtension {
    pub name: String,
    pub fields: Vec<InputFieldDefinition>,
    pub position: Position,
}

#[derive(Debug)]
pub struct EnumExtension {
    pub name: String,
    pub values: Vec<EnumValueDefinition>,
    pub position: Position,
}

#[derive(Debug)]
pub struct Position {
    pub file: String,
    /// One-based line number
    pub line: usize,
    /// One-based column number
    pub column: usize,
}

#[derive(Debug, Clone)]
pub enum TypeExpression {
    NamedType(String),
    ListType(Box<TypeExpression>),
    NonNullType(Box<TypeExpression>),
}

#[derive(Debug)]
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
