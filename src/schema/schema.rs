use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::{
    Enum, EnumExtension, EnumValue, Field, Input, InputExtension, InputValue, Interface,
    InterfaceTypeExtension, Object, ObjectExtension, Scalar, SourceConfig, Union, UnionExtension,
};

pub struct Schema {
    pub query: Option<String>,
    pub mutation: Option<String>,
    pub subscription: Option<String>,
    definition_locations: HashMap<String, Location>,
    extension_locations: HashMap<String, Vec<Location>>,
    elements: HashMap<PathBuf, Vec<SchemaElement>>,
}

impl Schema {
    pub fn new() -> Self {
        Self {
            query: None,
            mutation: None,
            subscription: None,
            definition_locations: HashMap::new(),
            extension_locations: HashMap::new(),
            elements: HashMap::new(),
        }
    }

    // pub fn get_query(&self) -> Option<&ObjectTypeDefinition> {
    //     None // Return None if "Query" definition is not found or not of ObjectTypeDefinition type
    // }

    pub fn get_definition(&self, name: &str) -> Option<&SchemaElement> {
        None // Return None if definition is not found
    }

    // Object
    pub fn collect_object_source_configs(&self, def: &Object) -> Vec<SourceConfig> {
        let mut cfs = def.collect_source_configs();
        for ext in self.iter_object_extensions(&def.name) {
            cfs.extend(ext.collect_source_configs());
        }
        cfs
    }

    pub fn collect_object_fields<'a>(&'a self, def: &'a Object) -> impl Iterator<Item = &'a Field> {
        def.fields.iter().chain(
            self.iter_object_extensions(&def.name)
                .flat_map(|ext| ext.fields.iter()),
        )
    }

    pub fn collect_object_interfaces<'a>(
        &'a self,
        def: &'a Object,
    ) -> impl Iterator<Item = &'a String> {
        def.interfaces.iter().chain(
            self.iter_object_extensions(&def.name)
                .flat_map(|ext| ext.interfaces.iter()),
        )
    }

    fn iter_object_extensions(&self, name: &str) -> impl Iterator<Item = &ObjectExtension> {
        self.extension_locations
            .get(name)
            .into_iter()
            .flat_map(|v| v.iter())
            .flat_map(move |loc| match &self.elements[&loc.path][loc.index] {
                SchemaElement::Extension(Extension::ObjectExtension(ext)) => Some(ext),
                _ => unreachable!(),
            })
    }

    // Interface
    pub fn collect_interface_fields<'a>(
        &'a self,
        def: &'a Interface,
    ) -> impl Iterator<Item = &'a Field> {
        def.fields.iter().chain(
            self.iter_interface_extensions(&def.name)
                .flat_map(|ext| ext.fields.iter()),
        )
    }

    pub fn collect_interface_source_configs(&self, def: &Interface) -> Vec<SourceConfig> {
        def.collect_source_configs()
    }

    pub fn collect_interface_interfaces<'a>(
        &'a self,
        def: &'a Interface,
    ) -> impl Iterator<Item = &'a String> {
        def.interfaces.iter().chain(
            self.iter_interface_extensions(&def.name)
                .flat_map(|ext| ext.interfaces.iter()),
        )
    }

    fn iter_interface_extensions(
        &self,
        name: &str,
    ) -> impl Iterator<Item = &InterfaceTypeExtension> {
        self.extension_locations
            .get(name)
            .into_iter()
            .flat_map(|v| v.iter())
            .flat_map(move |loc| match &self.elements[&loc.path][loc.index] {
                SchemaElement::Extension(Extension::InterfaceExtension(ext)) => Some(ext),
                _ => unreachable!(),
            })
    }

    // Input
    pub fn collect_input_fields<'a>(
        &'a self,
        def: &'a Input,
    ) -> impl Iterator<Item = &'a InputValue> {
        def.fields.iter().chain(
            self.iter_input_extensions(&def.name)
                .flat_map(|ext| ext.fields.iter()),
        )
    }

    pub fn collect_input_source_configs(&self, def: &Input) -> Vec<SourceConfig> {
        def.collect_source_configs()
    }

    fn iter_input_extensions(&self, name: &str) -> impl Iterator<Item = &InputExtension> {
        self.extension_locations
            .get(name)
            .into_iter()
            .flat_map(|v| v.iter())
            .flat_map(move |loc| match &self.elements[&loc.path][loc.index] {
                SchemaElement::Extension(Extension::InputExtension(ext)) => Some(ext),
                _ => unreachable!(),
            })
    }

    // Enum
    pub fn collect_enum_values<'a>(&'a self, def: &'a Enum) -> impl Iterator<Item = &'a EnumValue> {
        def.values.iter().chain(
            self.iter_enum_extensions(&def.name)
                .flat_map(|ext| ext.values.iter()),
        )
    }

    fn iter_enum_extensions(&self, name: &str) -> impl Iterator<Item = &EnumExtension> {
        self.extension_locations
            .get(name)
            .into_iter()
            .flat_map(|v| v.iter())
            .flat_map(move |loc| match &self.elements[&loc.path][loc.index] {
                SchemaElement::Extension(Extension::EnumExtension(ext)) => Some(ext),
                _ => unreachable!(),
            })
    }

    // Union
    pub fn collect_possible_types<'a>(
        &'a self,
        def: &'a Union,
    ) -> impl Iterator<Item = &'a String> {
        def.types.iter().chain(
            self.iter_union_extensions(&def.name)
                .flat_map(|ext| ext.types.iter()),
        )
    }

    fn iter_union_extensions(&self, name: &str) -> impl Iterator<Item = &UnionExtension> {
        self.extension_locations
            .get(name)
            .into_iter()
            .flat_map(|v| v.iter())
            .flat_map(move |loc| match &self.elements[&loc.path][loc.index] {
                SchemaElement::Extension(Extension::UnionExtension(ext)) => Some(ext),
                _ => unreachable!(),
            })
    }

    // Mut

    pub fn iter_definitions(&self) -> impl Iterator<Item = &Definition> {
        self.elements
            .values()
            .flat_map(|elements| elements.iter())
            .filter_map(|element| match element {
                SchemaElement::Definition(def) => Some(def),
                _ => None,
            })
    }

    pub fn add_definition(&mut self, path: &Path, def: Definition) {
        let name = def.get_name().to_owned();
        let element = SchemaElement::Definition(def);
        let container = self
            .elements
            .entry(path.to_owned())
            .or_insert_with(Vec::new);

        let location = Location {
            path: path.to_owned(),
            index: container.len(),
        };

        container.push(element);
        self.definition_locations.insert(name, location);
    }

    pub fn add_extension(&mut self, path: &Path, ext: Extension) {
        let name = ext.get_definition_name().to_owned();
        let element = SchemaElement::Extension(ext);
        let container = self
            .elements
            .entry(path.to_owned())
            .or_insert_with(Vec::new);

        let location = Location {
            path: path.to_owned(),
            index: container.len(),
        };

        container.push(element);
        self.extension_locations
            .entry(name)
            .or_insert_with(Vec::new)
            .push(location);
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Location {
    pub path: PathBuf,
    pub index: usize,
}

pub enum SchemaElement {
    Definition(Definition),
    Extension(Extension),
}

pub enum Definition {
    Scalar(Scalar),
    Object(Object),
    Interface(Interface),
    Union(Union),
    Enum(Enum),
    Input(Input),
}

impl Definition {
    pub fn get_name(&self) -> &str {
        match self {
            Definition::Scalar(scalar) => &scalar.name,
            Definition::Object(object) => &object.name,
            Definition::Interface(interface) => &interface.name,
            Definition::Union(union) => &union.name,
            Definition::Enum(enum_) => &enum_.name,
            Definition::Input(input) => &input.name,
        }
    }
}

impl From<Scalar> for Definition {
    fn from(scalar: Scalar) -> Self {
        Definition::Scalar(scalar)
    }
}
impl From<Object> for Definition {
    fn from(object: Object) -> Self {
        Definition::Object(object)
    }
}
impl From<Interface> for Definition {
    fn from(interface: Interface) -> Self {
        Definition::Interface(interface)
    }
}
impl From<Union> for Definition {
    fn from(union: Union) -> Self {
        Definition::Union(union)
    }
}
impl From<Enum> for Definition {
    fn from(enum_: Enum) -> Self {
        Definition::Enum(enum_)
    }
}
impl From<Input> for Definition {
    fn from(input: Input) -> Self {
        Definition::Input(input)
    }
}

pub enum Extension {
    ObjectExtension(ObjectExtension),
    InterfaceExtension(InterfaceTypeExtension),
    UnionExtension(UnionExtension),
    EnumExtension(EnumExtension),
    InputExtension(InputExtension),
}

impl Extension {
    pub fn get_definition_name(&self) -> &str {
        match self {
            Extension::ObjectExtension(ext) => &ext.name,
            Extension::InterfaceExtension(ext) => &ext.name,
            Extension::UnionExtension(ext) => &ext.name,
            Extension::EnumExtension(ext) => &ext.name,
            Extension::InputExtension(ext) => &ext.name,
        }
    }
}

impl From<ObjectExtension> for Extension {
    fn from(ext: ObjectExtension) -> Self {
        Extension::ObjectExtension(ext)
    }
}
impl From<InterfaceTypeExtension> for Extension {
    fn from(ext: InterfaceTypeExtension) -> Self {
        Extension::InterfaceExtension(ext)
    }
}
impl From<UnionExtension> for Extension {
    fn from(ext: UnionExtension) -> Self {
        Extension::UnionExtension(ext)
    }
}
impl From<EnumExtension> for Extension {
    fn from(ext: EnumExtension) -> Self {
        Extension::EnumExtension(ext)
    }
}
impl From<InputExtension> for Extension {
    fn from(ext: InputExtension) -> Self {
        Extension::InputExtension(ext)
    }
}
