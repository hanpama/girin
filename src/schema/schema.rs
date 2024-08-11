use core::panic;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::{
    Enum, EnumExtension, Input, InputExtension, Interface, InterfaceTypeExtension, Object,
    ObjectExtension, Scalar, Union, UnionExtension,
};

pub struct Schema {
    pub query: Option<String>,
    pub mutation: Option<String>,
    pub subscription: Option<String>,
    modules: HashMap<PathBuf, Module>,
    directories: HashMap<PathBuf, Directory>,
    definition_locations: HashMap<String, Location>,
    extension_locations: HashMap<String, Vec<Location>>,
    root_dir: PathBuf,
}

impl Schema {
    pub fn new() -> Self {
        Self {
            query: None,
            mutation: None,
            subscription: None,
            definition_locations: HashMap::new(),
            extension_locations: HashMap::new(),
            modules: HashMap::new(),
            directories: HashMap::new(),
            root_dir: PathBuf::new(),
        }
    }

    pub fn get_definition(&self, name: &str) -> Option<&Definition> {
        None
    }

    pub fn iter_all_definitions(&self) -> impl Iterator<Item = &Definition> {
        self.modules
            .values()
            .flat_map(|module| module.elements.iter())
            .filter_map(|element| match element {
                SchemaElement::Definition(def) => Some(def),
                _ => None,
            })
    }

    pub fn iter_extensions(&self, name: &str) -> impl Iterator<Item = &Extension> {
        self.extension_locations
            .get(name)
            .into_iter()
            .flat_map(|v| v.iter())
            .flat_map(move |loc| match self.get_element(&loc) {
                SchemaElement::Extension(ext) => Some(ext),
                _ => unreachable!(),
            })
    }

    pub fn add_definition(&mut self, dir: &Path, module_name: &str, def: Definition) {
        let name = def.get_name().to_owned();
        let element = SchemaElement::Definition(def);
        let location = self.add_element(dir, module_name, element);
        self.definition_locations.insert(name, location);
    }

    pub fn add_extension(&mut self, dir: &Path, module_name: &str, ext: Extension) {
        let name = ext.get_definition_name().to_owned();
        let element = SchemaElement::Extension(ext);
        let location = self.add_element(dir, module_name, element);
        self.extension_locations
            .entry(name)
            .or_insert_with(Vec::new)
            .push(location);
    }

    fn add_element(&mut self, dir: &Path, module: &str, element: SchemaElement) -> Location {
        let module = if let Some(module) = self.get_mut_module(dir, module) {
            module
        } else {
            self.add_module(dir, module)
        };

        module.add_element(element)
    }

    fn get_mut_module(&mut self, dir: &Path, module: &str) -> Option<&mut Module> {
        let path = dir.join(module);
        self.modules.get_mut(&path)
    }

    fn add_module(&mut self, dir_path: &Path, module_name: &str) -> &mut Module {
        let dir = if let Some(dir) = self.get_mut_directory(dir_path) {
            dir
        } else {
            self.add_directory(dir_path)
        };

        let module = Module::new(dir_path, module_name);
        let path = module.path.clone();

        dir.modules.push(path.clone());
        self.modules.insert(path.clone(), module);
        self.modules.get_mut(&path).unwrap()
    }

    fn get_mut_directory(&mut self, path: &Path) -> Option<&mut Directory> {
        self.directories.get_mut(path)
    }

    fn add_directory(&mut self, path: &Path) -> &mut Directory {
        if let Some(parent_path) = path.parent() {
            let parent = if let Some(parent) = self.get_mut_directory(parent_path) {
                parent
            } else {
                self.add_directory(parent_path)
            };
            parent.children.push(path.to_owned());
        }

        let dir = Directory::new(path.to_owned());
        self.directories.insert(path.to_owned(), dir);
        self.directories.get_mut(path).unwrap()
    }

    fn get_element(&self, location: &Location) -> &SchemaElement {
        &self.modules[&location.path].elements[location.index]
    }

    pub fn iter_child_modules(&self, path: &Path) -> impl Iterator<Item = &Module> {
        self.directories
            .get(path)
            .into_iter()
            .flat_map(|v| v.modules.iter())
            .map(|loc| &self.modules[loc])
    }

    pub fn iter_root_modules(&self) -> impl Iterator<Item = &Module> {
        self.iter_child_modules(&self.root_dir)
    }

    pub fn iter_child_directories(&self, path: &Path) -> impl Iterator<Item = &Directory> {
        self.directories
            .get(path)
            .into_iter()
            .flat_map(|v| v.children.iter())
            .map(|loc| &self.directories[loc])
    }

    pub fn iter_root_directories(&self) -> impl Iterator<Item = &Directory> {
        self.iter_child_directories(&self.root_dir)
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Location {
    pub path: PathBuf,
    pub index: usize,
}

pub struct Directory {
    pub path: PathBuf,
    pub children: Vec<PathBuf>,
    pub modules: Vec<PathBuf>,
}

impl Directory {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            children: Vec::new(),
            modules: Vec::new(),
        }
    }
}

pub struct Module {
    pub path: PathBuf,
    pub elements: Vec<SchemaElement>,
}

impl Module {
    pub fn new(dir: &Path, name: &str) -> Self {
        Self {
            path: dir.join(name),
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, element: SchemaElement) -> Location {
        let path = self.path.clone();
        let index = self.elements.len();
        self.elements.push(element);
        Location { path, index }
    }
}

pub enum SchemaElement {
    Definition(Definition),
    Extension(Extension),
}

impl From<Definition> for SchemaElement {
    fn from(def: Definition) -> Self {
        SchemaElement::Definition(def)
    }
}

impl From<Extension> for SchemaElement {
    fn from(ext: Extension) -> Self {
        SchemaElement::Extension(ext)
    }
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
