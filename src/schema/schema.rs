use std::hash::Hash;
use std::{collections::HashMap, iter::once};

use super::{definition, Definition, ObjectExtension, SchemaDefinition};

pub struct Schema {
    definitions: HashMap<String, Loc>,
    extensions: HashMap<String, Vec<Loc>>,

    pub root_dir: Directory,
}

impl Schema {
    pub fn new(root_dir: Directory) -> Self {
        Self {
            definitions: HashMap::new(),
            extensions: HashMap::new(),
            root_dir,
        }
    }

    pub fn traverse_definitions(&self) -> Vec<Traversal<Definition>> {
        self.root_dir.iter_definition_locs()
    }

    pub fn traverse_object_extensions(&self, name: &str) -> Vec<Traversal<ObjectExtension>> {
        self.traverse_extensions(name)
            .iter()
            .map(|t| t.definition.into())
            .collect()
    }

    fn traverse_extensions<T: From<Definition>>(&self, name: &str) -> Vec<Traversal<T>> {
        self.extensions
            .get(name)
            .into_iter()
            .flat_map(|locs| {
                locs.iter()
                    .map(move |loc| self.get_traversal_node(loc.clone(), name))
                // .map()
            })
            .collect()
    }

    // fn get_traversal_node(&self, loc: Loc) -> {

    // }
}

pub struct Directory {
    pub name: String,
    directories: Vec<Directory>,
    modules: Vec<Module>,
}

pub enum DirectoryChild {
    Directory(Directory),
    Module(Module),
}

impl Directory {
    pub fn new(name: String) -> Self {
        Self {
            name,
            directories: Vec::new(),
            modules: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_directory(&self, name: &str) -> &Directory {
        self.directories
            .iter()
            .find(|d| d.get_name() == name)
            .unwrap()
    }

    pub fn add_directory(&mut self, dir: Directory) {
        self.directories.push(dir);
    }

    pub fn iter_directories(&self) -> impl Iterator<Item = &Directory> {
        self.directories.iter()
    }

    pub fn add_module(&mut self, module: Module) {
        self.modules.push(module);
    }

    pub fn get_module(&self, name: &str) -> &Module {
        self.modules.iter().find(|m| m.get_name() == name).unwrap()
    }

    pub fn iter_modules(&self) -> impl Iterator<Item = &Module> {
        self.modules.iter()
    }

    pub fn iter_definition_locs<'a>(&'a self) -> Box<dyn Iterator<Item = Loc> + 'a> {
        Box::new(
            self.directories
                .iter()
                .flat_map(Directory::iter_definition_locs)
                .chain(self.modules.iter().flat_map(Module::iter_definition_locs))
                .map(move |loc| loc.prepend(&self.name)),
        )
    }
}

pub struct Module {
    name: String,
    schema_definition: Option<SchemaDefinition>,
    definitions: Vec<Definition>,
    definition_map: HashMap<String, usize>,
}

impl Module {
    pub fn new(name: String) -> Self {
        Self {
            name,
            schema_definition: None,
            definitions: Vec::new(),
            definition_map: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_schema_definition(&mut self, schema_definition: SchemaDefinition) {
        self.schema_definition = Some(schema_definition);
    }

    pub fn get_definition(&self, name: &str) -> &Definition {
        let idx = *self.definition_map.get(name).unwrap();
        &self.definitions[idx]
    }

    pub fn add_definition(&mut self, definition: Definition) {
        self.definition_map
            .insert(definition.get_name().to_owned(), self.definitions.len());
        self.definitions.push(definition);
    }

    pub fn iter_definitions(&self) -> impl Iterator<Item = &Definition> {
        self.definitions.iter()
    }

    pub fn iter_definition_locs<'a>(&'a self) -> impl Iterator<Item = Loc> + 'a {
        self.definitions
            .iter()
            .map(|def| Loc::new(&self.name, def.get_name()))
    }
}

struct Loc<'a> {
    directory: Vec<String>,
    module: String,
    definition: String,
}

impl Loc {
    pub fn new(module: &str, definition: &str) -> Self {
        Self {
            directory: Vec::new(),
            module: module.to_owned(),
            definition: definition.to_owned(),
        }
    }

    fn prepend(mut self, name: &str) -> Loc {
        self.directory.insert(0, name.to_owned());
        self
    }
}

// pub struct Traversal<'a, T> {
//     context: Loc,
//     current: &'a T,
// }

// impl<'a, T> Traversal<'a, T> {
//     pub fn new(context: Loc, current: &'a T) -> Traversal<'a, T> {
//         Self { context, current }
//     }

//     pub fn on(mut self, name: &str) -> Traversal<'a, T> {
//         self.context.insert(0, name.to_owned());
//         self
//     }
// }
