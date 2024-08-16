use std::{
    collections::HashMap,
    iter::{empty, once},
};

use super::{Definition, ExtensionRef, Module};

#[derive(Debug)]
pub struct Directory {
    pub name: String,
    children: Vec<DirectoryChild>,
    directories: HashMap<String, usize>,
    modules: HashMap<String, usize>,
    def_idx: HashMap<String, String>,
    ext_idx: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
pub enum DirectoryChild {
    Directory(Directory),
    Module(Module),
}

impl Directory {
    pub fn new(name: String) -> Self {
        Self {
            name,
            children: Vec::new(),
            directories: HashMap::new(),
            modules: HashMap::new(),
            def_idx: HashMap::new(),
            ext_idx: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_directory(&self, name: &str) -> &Directory {
        match &self.children[*self.directories.get(name).unwrap()] {
            DirectoryChild::Directory(dir) => dir,
            _ => unreachable!(),
        }
    }

    pub fn add_directory(&mut self, dir: Directory) {
        for def_name in dir.def_idx.keys() {
            self.def_idx
                .insert(def_name.to_owned(), dir.get_name().to_owned());
        }
        for ext_name in dir.ext_idx.keys() {
            self.ext_idx
                .entry(ext_name.to_owned())
                .or_insert_with(Vec::new)
                .push(dir.get_name().to_owned());
        }
        self.directories
            .insert(dir.get_name().to_owned(), self.children.len());
        self.children.push(DirectoryChild::Directory(dir));
    }

    pub fn iter_directories(&self) -> impl Iterator<Item = &Directory> {
        self.directories
            .values()
            .map(move |&idx| match &self.children[idx] {
                DirectoryChild::Directory(dir) => dir,
                _ => unreachable!(),
            })
    }

    pub fn add_module(&mut self, module: Module) {
        for def in module.iter_definitions() {
            self.def_idx
                .insert(def.get_name().to_owned(), module.get_name().to_owned());
        }
        for ext in module.iter_extensions() {
            self.ext_idx
                .entry(ext.get_name().to_owned())
                .or_insert_with(Vec::new)
                .push(module.get_name().to_owned());
        }

        self.modules
            .insert(module.get_name().to_owned(), self.children.len());
        self.children.push(DirectoryChild::Module(module));
    }

    pub fn get_module(&self, name: &str) -> &Module {
        match &self.children[*self.modules.get(name).unwrap()] {
            DirectoryChild::Module(module) => module,
            _ => unreachable!(),
        }
    }

    pub fn iter_modules(&self) -> impl Iterator<Item = &Module> {
        self.modules
            .values()
            .map(move |&idx| match &self.children[idx] {
                DirectoryChild::Module(module) => module,
                _ => unreachable!(),
            })
    }

    pub fn iter_definitions(&self) -> impl Iterator<Item = &Definition> {
        self.def_idx.keys().map(|name| self.get_definition(name))
    }

    pub fn get_definition(&self, name: &str) -> &Definition {
        match self.get_child(&self.def_idx[name]) {
            DirectoryChild::Directory(dir) => dir.get_definition(name),
            DirectoryChild::Module(module) => module.get_definition(name),
        }
    }

    pub fn resolve_extensions<'a>(
        &'a self,
        name: &'a str,
    ) -> Box<dyn Iterator<Item = ExtensionRef<'a>> + 'a> {
        if !self.ext_idx.contains_key(name) {
            return Box::new(empty());
        }
        Box::new(
            self.ext_idx[name]
                .iter()
                .map(|n| self.get_child(n))
                .flat_map(|child| match child {
                    DirectoryChild::Directory(dir) => dir.resolve_extensions(name),
                    DirectoryChild::Module(module) => Box::new(once(ExtensionRef::new(
                        module.get_name(),
                        module.get_extension(name),
                    ))),
                })
                .map(|loc| loc.prepend(&self.name)),
        )
    }

    fn get_child<'a>(&'a self, seg: &'a str) -> &'a DirectoryChild {
        if let Some(idx) = self.directories.get(seg) {
            return &self.children[*idx];
        } else {
            return &self.children[self.modules[seg]];
        }
    }
}
