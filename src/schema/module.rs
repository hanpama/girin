use std::collections::HashMap;

use super::{Definition, Extension, SchemaDefinition};

pub struct Module {
    name: String,
    schema_definition: Option<SchemaDefinition>,
    children: Vec<ModuleChild>,
    def_idx: HashMap<String, usize>,
    ext_idx: HashMap<String, usize>,
}

pub enum ModuleChild {
    Definition(Definition),
    Extension(Extension),
}

impl Module {
    pub fn new(name: String) -> Self {
        Self {
            name,
            schema_definition: None,
            children: Vec::new(),
            def_idx: HashMap::new(),
            ext_idx: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_schema_definition(&mut self, schema_definition: SchemaDefinition) {
        self.schema_definition = Some(schema_definition);
    }

    pub fn get_definition(&self, name: &str) -> &Definition {
        match &self.children[*self.def_idx.get(name).unwrap()] {
            ModuleChild::Definition(def) => def,
            _ => unreachable!(),
        }
    }

    pub fn add_definition(&mut self, definition: Definition) {
        self.def_idx
            .insert(definition.get_name().to_owned(), self.children.len());
        self.children.push(ModuleChild::Definition(definition));
    }

    pub fn iter_definitions(&self) -> impl Iterator<Item = &Definition> {
        self.def_idx
            .values()
            .map(move |&idx| match &self.children[idx] {
                ModuleChild::Definition(def) => def,
                _ => unreachable!(),
            })
    }

    pub fn get_extension(&self, name: &str) -> &Extension {
        match &self.children[*self.ext_idx.get(name).unwrap()] {
            ModuleChild::Extension(ext) => ext,
            _ => unreachable!(),
        }
    }

    pub fn add_extension(&mut self, extension: Extension) {
        self.ext_idx
            .insert(extension.get_name().to_owned(), self.children.len());
        self.children.push(ModuleChild::Extension(extension));
    }

    pub fn iter_extensions(&self) -> impl Iterator<Item = &Extension> {
        self.ext_idx
            .values()
            .map(move |&idx| match &self.children[idx] {
                ModuleChild::Extension(ext) => ext,
                _ => unreachable!(),
            })
    }
}
