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
    root_dir: Directory,
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
            root_dir: Directory::new(PathBuf::new()),
        }
    }

    pub fn get_definition(&self, name: &str) -> Option<&TypeDefinition> {
        None
    }

    pub fn iter_all_definitions(&self) -> impl Iterator<Item = &TypeDefinition> {
        self.modules
            .values()
            .flat_map(|module| module.definitions.iter())
            .filter_map(|element| match element {
                Definition::TypeDefinition(def) => Some(def),
                _ => None,
            })
    }

    pub fn iter_extensions(&self, name: &str) -> impl Iterator<Item = &TypeExtension> {
        self.extension_locations
            .get(name)
            .into_iter()
            .flat_map(|v| v.iter())
            .flat_map(move |loc| match self.get_element(&loc) {
                Definition::TypeExtension(ext) => Some(ext),
                _ => unreachable!(),
            })
    }

    pub fn add_definition(&mut self, dir: &Path, module_name: &str, def: TypeDefinition) {
        let name = def.get_name().to_owned();
        let element = Definition::TypeDefinition(def);
        let location = self.add_element(dir, module_name, element);
        self.definition_locations.insert(name, location);
    }

    pub fn add_extension(&mut self, dir: &Path, module_name: &str, ext: TypeExtension) {
        let name = ext.get_definition_name().to_owned();
        let element = Definition::TypeExtension(ext);
        let location = self.add_element(dir, module_name, element);
        self.extension_locations
            .entry(name)
            .or_insert_with(Vec::new)
            .push(location);
    }

    // pub fn collect_object_extensions(
    //     &self,
    //     object: &Object,
    // ) -> impl Iterator<Item = &ObjectExtension> {
    //     self.iter_extensions(&object.name)
    //         .filter_map(|ext| match ext {
    //             TypeExtension::ObjectExtension(ext) => Some(ext),
    //             _ => None,
    //         })
    // }

    // pub fn get_definition_module(&self, name: &str) -> &Module {
    //     self.definition_locations
    //         .get(name)
    //         .map(|loc| &self.modules[&loc.path])
    //         .unwrap()
    // }

    fn add_element(&mut self, dir: &Path, module: &str, element: Definition) -> Location {
        let module = if let Some(module) = self.get_mut_module(dir, module) {
            module
        } else {
            self.add_module(dir, module)
        };

        module.add_definition(element)
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

    fn get_element(&self, location: &Location) -> &Definition {
        &self.modules[&location.path].definitions[location.index]
    }

    pub fn iter_child_modules(&self, dir: &Directory) -> impl Iterator<Item = &Module> {
        self.directories
            .get(&dir.path)
            .into_iter()
            .flat_map(|v| v.modules.iter())
            .map(|loc| &self.modules[loc])
    }

    pub fn iter_root_modules(&self) -> impl Iterator<Item = &Module> {
        self.iter_child_modules(&self.root_dir)
    }

    pub fn iter_child_directories(&self, dir: &Directory) -> impl Iterator<Item = &Directory> {
        self.directories
            .get(&dir.path)
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
    pub definitions: Vec<Definition>,
}

impl Module {
    pub fn new(dir: &Path, name: &str) -> Self {
        Self {
            path: dir.join(name),
            definitions: Vec::new(),
        }
    }

    fn add_definition(&mut self, definition: Definition) -> Location {
        let path = self.path.clone();
        let index = self.definitions.len();
        self.definitions.push(definition);
        Location { path, index }
    }
}

pub enum Definition {
    TypeDefinition(TypeDefinition),
    TypeExtension(TypeExtension),
}

impl From<TypeDefinition> for Definition {
    fn from(def: TypeDefinition) -> Self {
        Definition::TypeDefinition(def)
    }
}

impl From<TypeExtension> for Definition {
    fn from(ext: TypeExtension) -> Self {
        Definition::TypeExtension(ext)
    }
}

pub enum TypeDefinition {
    Scalar(Scalar),
    Object(Object),
    Interface(Interface),
    Union(Union),
    Enum(Enum),
    Input(Input),
}

impl TypeDefinition {
    pub fn get_name(&self) -> &str {
        match self {
            TypeDefinition::Scalar(scalar) => &scalar.name,
            TypeDefinition::Object(object) => &object.name,
            TypeDefinition::Interface(interface) => &interface.name,
            TypeDefinition::Union(union) => &union.name,
            TypeDefinition::Enum(enum_) => &enum_.name,
            TypeDefinition::Input(input) => &input.name,
        }
    }
}

impl From<Scalar> for TypeDefinition {
    fn from(scalar: Scalar) -> Self {
        TypeDefinition::Scalar(scalar)
    }
}
impl From<Object> for TypeDefinition {
    fn from(object: Object) -> Self {
        TypeDefinition::Object(object)
    }
}
impl From<Interface> for TypeDefinition {
    fn from(interface: Interface) -> Self {
        TypeDefinition::Interface(interface)
    }
}
impl From<Union> for TypeDefinition {
    fn from(union: Union) -> Self {
        TypeDefinition::Union(union)
    }
}
impl From<Enum> for TypeDefinition {
    fn from(enum_: Enum) -> Self {
        TypeDefinition::Enum(enum_)
    }
}
impl From<Input> for TypeDefinition {
    fn from(input: Input) -> Self {
        TypeDefinition::Input(input)
    }
}

pub enum TypeExtension {
    ObjectExtension(ObjectExtension),
    InterfaceExtension(InterfaceTypeExtension),
    UnionExtension(UnionExtension),
    EnumExtension(EnumExtension),
    InputExtension(InputExtension),
}

impl TypeExtension {
    pub fn get_definition_name(&self) -> &str {
        match self {
            TypeExtension::ObjectExtension(ext) => &ext.name,
            TypeExtension::InterfaceExtension(ext) => &ext.name,
            TypeExtension::UnionExtension(ext) => &ext.name,
            TypeExtension::EnumExtension(ext) => &ext.name,
            TypeExtension::InputExtension(ext) => &ext.name,
        }
    }
}

impl From<ObjectExtension> for TypeExtension {
    fn from(ext: ObjectExtension) -> Self {
        TypeExtension::ObjectExtension(ext)
    }
}
impl From<InterfaceTypeExtension> for TypeExtension {
    fn from(ext: InterfaceTypeExtension) -> Self {
        TypeExtension::InterfaceExtension(ext)
    }
}
impl From<UnionExtension> for TypeExtension {
    fn from(ext: UnionExtension) -> Self {
        TypeExtension::UnionExtension(ext)
    }
}
impl From<EnumExtension> for TypeExtension {
    fn from(ext: EnumExtension) -> Self {
        TypeExtension::EnumExtension(ext)
    }
}
impl From<InputExtension> for TypeExtension {
    fn from(ext: InputExtension) -> Self {
        TypeExtension::InputExtension(ext)
    }
}
