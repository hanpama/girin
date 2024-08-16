use super::{
    EnumDefinition, EnumExtension, InputDefinition, InputExtension, InterfaceDefinition,
    InterfaceExtension, ObjectDefinition, ObjectExtension, ScalarDefinition, UnionDefinition,
    UnionExtension,
};

#[derive(Debug)]
pub enum Definition {
    ScalarDefinition(ScalarDefinition),
    ObjectDefinition(ObjectDefinition),
    InterfaceDefinition(InterfaceDefinition),
    UnionDefinition(UnionDefinition),
    EnumDefinition(EnumDefinition),
    InputDefinition(InputDefinition),
}

impl Definition {
    pub fn get_name(&self) -> &str {
        match self {
            Definition::ScalarDefinition(scalar) => &scalar.name,
            Definition::ObjectDefinition(object) => &object.name,
            Definition::InterfaceDefinition(interface) => &interface.name,
            Definition::UnionDefinition(union) => &union.name,
            Definition::EnumDefinition(enm) => &enm.name,
            Definition::InputDefinition(input) => &input.name,
        }
    }

    pub fn as_scalar(&self) -> Option<&ScalarDefinition> {
        match self {
            Definition::ScalarDefinition(scalar) => Some(scalar),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&ObjectDefinition> {
        match self {
            Definition::ObjectDefinition(object) => Some(object),
            _ => None,
        }
    }

    pub fn as_interface(&self) -> Option<&InterfaceDefinition> {
        match self {
            Definition::InterfaceDefinition(interface) => Some(interface),
            _ => None,
        }
    }

    pub fn as_union(&self) -> Option<&UnionDefinition> {
        match self {
            Definition::UnionDefinition(union) => Some(union),
            _ => None,
        }
    }

    pub fn as_enum(&self) -> Option<&EnumDefinition> {
        match self {
            Definition::EnumDefinition(enm) => Some(enm),
            _ => None,
        }
    }

    pub fn as_input(&self) -> Option<&InputDefinition> {
        match self {
            Definition::InputDefinition(input) => Some(input),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Extension {
    ObjectExtension(ObjectExtension),
    InterfaceExtension(InterfaceExtension),
    UnionExtension(UnionExtension),
    EnumExtension(EnumExtension),
    InputExtension(InputExtension),
}

impl Extension {
    pub fn get_name(&self) -> &str {
        match self {
            Extension::ObjectExtension(object) => &object.name,
            Extension::InterfaceExtension(interface) => &interface.name,
            Extension::UnionExtension(union) => &union.name,
            Extension::EnumExtension(enm) => &enm.name,
            Extension::InputExtension(input) => &input.name,
        }
    }

    pub fn as_object(&self) -> Option<&ObjectExtension> {
        match self {
            Extension::ObjectExtension(object) => Some(object),
            _ => None,
        }
    }

    pub fn as_interface(&self) -> Option<&InterfaceExtension> {
        match self {
            Extension::InterfaceExtension(interface) => Some(interface),
            _ => None,
        }
    }

    pub fn as_union(&self) -> Option<&UnionExtension> {
        match self {
            Extension::UnionExtension(union) => Some(union),
            _ => None,
        }
    }

    pub fn as_enum(&self) -> Option<&EnumExtension> {
        match self {
            Extension::EnumExtension(enm) => Some(enm),
            _ => None,
        }
    }

    pub fn as_input(&self) -> Option<&InputExtension> {
        match self {
            Extension::InputExtension(input) => Some(input),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct ExtensionRef<'a> {
    pub directory: Vec<&'a str>,
    pub module: &'a str,
    pub extension: &'a Extension,
}

impl<'a> ExtensionRef<'a> {
    pub fn new(module: &'a str, extension: &'a Extension) -> Self {
        Self {
            directory: Vec::new(),
            module,
            extension,
        }
    }

    pub fn prepend(mut self, name: &'a str) -> ExtensionRef<'a> {
        self.directory.insert(0, name);
        self
    }
}
