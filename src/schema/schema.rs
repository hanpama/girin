use super::{
    Definition, Directory, EnumExtension, Extension, InputExtension, InterfaceExtension, Loc,
    Module, ObjectExtension, UnionExtension,
};

pub struct Schema {
    pub root_dir: Directory,
}

impl Schema {
    pub fn new(root_dir: Directory) -> Self {
        Self { root_dir }
    }

    pub fn iter_definitions(&self) -> impl Iterator<Item = &Definition> {
        self.root_dir.iter_definitions()
    }

    pub fn collect_object_exts(&self, name: &str) -> impl Iterator<Item = (Loc, &ObjectExtension)> {
        self.collect_exts(name).map(|(loc, def)| match def {
            Extension::ObjectExtension(ext) => (loc, ext),
            _ => unreachable!(),
        })
    }

    pub fn iter_interface_exts(
        &self,
        name: &str,
    ) -> impl Iterator<Item = (&Loc, &InterfaceExtension)> {
        self.collect_exts(name).map(|(loc, def)| match def {
            Extension::InterfaceExtension(ext) => (loc, ext),
            _ => unreachable!(),
        })
    }

    pub fn iter_union_exts(&self, name: &str) -> impl Iterator<Item = (&Loc, &UnionExtension)> {
        self.collect_exts(name).map(|(loc, def)| match def {
            Extension::UnionExtension(ext) => (loc, ext),
            _ => unreachable!(),
        })
    }

    pub fn iter_enum_exts(&self, name: &str) -> impl Iterator<Item = (&Loc, &EnumExtension)> {
        self.collect_exts(name).map(|(loc, def)| match def {
            Extension::EnumExtension(ext) => (loc, ext),
            _ => unreachable!(),
        })
    }

    pub fn iter_input_exts(&self, name: &str) -> impl Iterator<Item = (Loc, &InputExtension)> {
        self.collect_exts(name).iter().map(|(loc, def)| match def {
            Extension::InputExtension(ext) => (loc, ext),
            _ => unreachable!(),
        })
    }

    fn collect_exts<'a>(&'a self, name: &'a str) -> Vec<(Loc<'a>, &'a Extension)> {
        self.root_dir.collect_extension(name)
    }

    pub fn resolve_directory(&self, loc: &Loc) -> &Directory {
        let mut dir = &self.root_dir;
        for name in &loc.directory {
            dir = dir.get_directory(name);
        }
        dir
    }

    pub fn resolve_module(&self, loc: &Loc) -> &Module {
        self.resolve_directory(loc).get_module(loc.module)
    }

    pub fn resolve_definition(&self, loc: &Loc) -> &Definition {
        self.resolve_module(loc).get_definition(loc.definition)
    }
}
