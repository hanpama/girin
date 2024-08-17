use std::iter::empty;

use super::{
    EnumDefinition, EnumExtension, Field, InputDefinition, InputExtension, InterfaceDefinition,
    InterfaceExtension, Module, ObjectDefinition, ObjectExtension, ScalarDefinition,
    UnionDefinition, UnionExtension,
};

#[derive(Debug)]
pub enum Type {
    Definition(Definition),
    Extension(Extension),
}

impl Type {
    pub fn get_name(&self) -> &str {
        match self {
            Type::Definition(def) => def.get_name(),
            Type::Extension(ext) => ext.get_name(),
        }
    }

    pub fn get_module(&self) -> &Module {
        match self {
            Type::Definition(def) => def.get_module(),
            Type::Extension(ext) => ext.get_module(),
        }
    }

    pub fn as_definition(&self) -> &Definition {
        match self {
            Type::Definition(def) => def,
            _ => unreachable!(),
        }
    }

    pub fn as_extension(&self) -> &Extension {
        match self {
            Type::Extension(ext) => ext,
            _ => unreachable!(),
        }
    }

    pub fn get_field(&self, name: &str) -> Option<&Field> {
        match self {
            Type::Definition(def) => def.get_field(name),
            Type::Extension(ext) => ext.get_field(name),
        }
    }

    pub fn iter_fields(&self) -> impl Iterator<Item = &Field> {
        match self {
            Type::Definition(def) => def.iter_fields(),
            Type::Extension(ext) => ext.iter_fields(),
        }
    }

    pub fn iter_interfaces(&self) -> impl Iterator<Item = &str> {
        match self {
            Type::Definition(def) => def.iter_interfaces(),
            Type::Extension(ext) => ext.iter_interfaces(),
        }
    }
}

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
    pub fn get_module(&self) -> &Module {
        match self {
            Definition::ScalarDefinition(scalar) => &scalar.module,
            Definition::ObjectDefinition(object) => &object.module,
            Definition::InterfaceDefinition(interface) => &interface.module,
            Definition::UnionDefinition(union) => &union.module,
            Definition::EnumDefinition(enm) => &enm.module,
            Definition::InputDefinition(input) => &input.module,
        }
    }

    pub fn as_scalar(&self) -> &ScalarDefinition {
        match self {
            Definition::ScalarDefinition(scalar) => scalar,
            _ => unreachable!(),
        }
    }

    pub fn as_object(&self) -> &ObjectDefinition {
        match self {
            Definition::ObjectDefinition(object) => object,
            _ => unreachable!(),
        }
    }

    pub fn as_interface(&self) -> &InterfaceDefinition {
        match self {
            Definition::InterfaceDefinition(interface) => interface,
            _ => unreachable!(),
        }
    }

    pub fn as_union(&self) -> &UnionDefinition {
        match self {
            Definition::UnionDefinition(union) => union,
            _ => unreachable!(),
        }
    }

    pub fn as_enum(&self) -> &EnumDefinition {
        match self {
            Definition::EnumDefinition(enm) => enm,
            _ => unreachable!(),
        }
    }

    pub fn as_input(&self) -> &InputDefinition {
        match self {
            Definition::InputDefinition(input) => input,
            _ => unreachable!(),
        }
    }

    pub fn get_field(&self, name: &str) -> Option<&Field> {
        match self {
            Definition::ObjectDefinition(object) => object.get_field(name),
            Definition::InterfaceDefinition(interface) => interface.get_field(name),
            _ => None,
        }
    }

    pub fn iter_fields<'a>(&'a self) -> Box<dyn Iterator<Item = &Field> + 'a> {
        match self {
            Definition::ObjectDefinition(object) => Box::new(object.iter_fields()),
            Definition::InterfaceDefinition(interface) => Box::new(interface.iter_fields()),
            _ => Box::new(empty()),
        }
    }

    pub fn iter_interfaces<'a>(&'a self) -> Box<dyn Iterator<Item = &str> + 'a> {
        match self {
            Definition::ObjectDefinition(object) => Box::new(object.iter_interfaces()),
            Definition::InterfaceDefinition(interface) => Box::new(interface.iter_interfaces()),
            _ => Box::new(empty()),
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

    pub fn get_module(&self) -> &Module {
        match self {
            Extension::ObjectExtension(object) => &object.module,
            Extension::InterfaceExtension(interface) => &interface.module,
            Extension::UnionExtension(union) => &union.module,
            Extension::EnumExtension(enm) => &enm.module,
            Extension::InputExtension(input) => &input.module,
        }
    }

    pub fn as_object_ext(&self) -> &ObjectExtension {
        match self {
            Extension::ObjectExtension(object) => object,
            _ => unreachable!(),
        }
    }

    pub fn as_interface_ext(&self) -> &InterfaceExtension {
        match self {
            Extension::InterfaceExtension(interface) => interface,
            _ => unreachable!(),
        }
    }

    pub fn as_union_ext(&self) -> &UnionExtension {
        match self {
            Extension::UnionExtension(union) => union,
            _ => unreachable!(),
        }
    }

    pub fn as_enum_ext(&self) -> &EnumExtension {
        match self {
            Extension::EnumExtension(enm) => enm,
            _ => unreachable!(),
        }
    }

    pub fn as_input_ext(&self) -> &InputExtension {
        match self {
            Extension::InputExtension(input) => input,
            _ => unreachable!(),
        }
    }

    pub fn get_field(&self, name: &str) -> Option<&Field> {
        match self {
            Extension::ObjectExtension(object) => object.get_field(name),
            Extension::InterfaceExtension(interface) => interface.get_field(name),
            _ => None,
        }
    }

    pub fn iter_fields<'a>(&'a self) -> Box<dyn Iterator<Item = &Field> + 'a> {
        match self {
            Extension::ObjectExtension(object) => Box::new(object.iter_fields()),
            Extension::InterfaceExtension(interface) => Box::new(interface.iter_fields()),
            _ => Box::new(empty()),
        }
    }

    pub fn iter_interfaces<'a>(&'a self) -> Box<dyn Iterator<Item = &str> + 'a> {
        match self {
            Extension::ObjectExtension(object) => Box::new(object.iter_interfaces()),
            Extension::InterfaceExtension(interface) => Box::new(interface.iter_interfaces()),
            _ => Box::new(empty()),
        }
    }
}
