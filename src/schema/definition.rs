use std::iter::empty;

use super::{
    DirectiveDefinition, EnumDefinition, EnumExtension, Field, InputDefinition, InputExtension,
    InterfaceDefinition, InterfaceExtension, ObjectDefinition, ObjectExtension, ScalarDefinition,
    SchemaDefinition, UnionDefinition, UnionExtension,
};

#[derive(Debug)]
pub enum Definition {
    SchemaDefinition(SchemaDefinition),
    DirectiveDefinition(DirectiveDefinition),
    ScalarDefinition(ScalarDefinition),
    ObjectDefinition(ObjectDefinition),
    InterfaceDefinition(InterfaceDefinition),
    UnionDefinition(UnionDefinition),
    EnumDefinition(EnumDefinition),
    InputDefinition(InputDefinition),
    ObjectExtension(ObjectExtension),
    InterfaceExtension(InterfaceExtension),
    UnionExtension(UnionExtension),
    EnumExtension(EnumExtension),
    InputExtension(InputExtension),
}

impl Definition {
    pub fn get_definition_name(&self) -> Option<&str> {
        match self {
            Definition::SchemaDefinition(_) => None,
            Definition::DirectiveDefinition(_) => None,
            Definition::ScalarDefinition(scalar) => Some(&scalar.name),
            Definition::ObjectDefinition(object) => Some(&object.name),
            Definition::InterfaceDefinition(interface) => Some(&interface.name),
            Definition::UnionDefinition(union) => Some(&union.name),
            Definition::EnumDefinition(enm) => Some(&enm.name),
            Definition::InputDefinition(input) => Some(&input.name),
            Definition::ObjectExtension(object) => Some(&object.name),
            Definition::InterfaceExtension(interface) => Some(&interface.name),
            Definition::UnionExtension(union) => Some(&union.name),
            Definition::EnumExtension(enm) => Some(&enm.name),
            Definition::InputExtension(input) => Some(&input.name),
        }
    }

    pub fn is_type_definition(&self) -> bool {
        match self {
            Definition::ScalarDefinition(_) => true,
            Definition::ObjectDefinition(_) => true,
            Definition::InterfaceDefinition(_) => true,
            Definition::UnionDefinition(_) => true,
            Definition::EnumDefinition(_) => true,
            Definition::InputDefinition(_) => true,
            _ => false,
        }
    }

    pub fn is_type_extension(&self) -> bool {
        match self {
            Definition::ObjectExtension(_) => true,
            Definition::InterfaceExtension(_) => true,
            Definition::UnionExtension(_) => true,
            Definition::EnumExtension(_) => true,
            Definition::InputExtension(_) => true,
            _ => false,
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

    pub fn as_object_ext(&self) -> &ObjectExtension {
        match self {
            Definition::ObjectExtension(object) => object,
            _ => unreachable!(),
        }
    }

    pub fn as_interface_ext(&self) -> &InterfaceExtension {
        match self {
            Definition::InterfaceExtension(interface) => interface,
            _ => unreachable!(),
        }
    }

    pub fn as_union_ext(&self) -> &UnionExtension {
        match self {
            Definition::UnionExtension(union) => union,
            _ => unreachable!(),
        }
    }

    pub fn as_enum_ext(&self) -> &EnumExtension {
        match self {
            Definition::EnumExtension(enm) => enm,
            _ => unreachable!(),
        }
    }

    pub fn as_input_ext(&self) -> &InputExtension {
        match self {
            Definition::InputExtension(input) => input,
            _ => unreachable!(),
        }
    }

    pub fn get_field(&self, name: &str) -> Option<&Field> {
        match self {
            Definition::ObjectDefinition(object) => object.get_field(name),
            Definition::InterfaceDefinition(interface) => interface.get_field(name),
            Definition::ObjectExtension(object) => object.get_field(name),
            Definition::InterfaceExtension(interface) => interface.get_field(name),
            _ => None,
        }
    }

    pub fn iter_fields<'a>(&'a self) -> Box<dyn Iterator<Item = &Field> + 'a> {
        match self {
            Definition::ObjectDefinition(object) => Box::new(object.iter_fields()),
            Definition::InterfaceDefinition(interface) => Box::new(interface.iter_fields()),
            Definition::ObjectExtension(object) => Box::new(object.iter_fields()),
            Definition::InterfaceExtension(interface) => Box::new(interface.iter_fields()),
            _ => Box::new(empty()),
        }
    }

    pub fn iter_interfaces<'a>(&'a self) -> Box<dyn Iterator<Item = &str> + 'a> {
        match self {
            Definition::ObjectDefinition(object) => Box::new(object.iter_interfaces()),
            Definition::InterfaceDefinition(interface) => Box::new(interface.iter_interfaces()),
            Definition::ObjectExtension(object) => Box::new(object.iter_interfaces()),
            Definition::InterfaceExtension(interface) => Box::new(interface.iter_interfaces()),
            _ => Box::new(empty()),
        }
    }

    pub fn get_position(&self) -> &super::Position {
        match self {
            Definition::SchemaDefinition(schema) => &schema.position,
            Definition::DirectiveDefinition(directive) => &directive.position,
            Definition::ScalarDefinition(scalar) => &scalar.position,
            Definition::ObjectDefinition(object) => &object.position,
            Definition::InterfaceDefinition(interface) => &interface.position,
            Definition::UnionDefinition(union) => &union.position,
            Definition::EnumDefinition(enm) => &enm.position,
            Definition::InputDefinition(input) => &input.position,
            Definition::ObjectExtension(object) => &object.position,
            Definition::InterfaceExtension(interface) => &interface.position,
            Definition::UnionExtension(union) => &union.position,
            Definition::EnumExtension(enm) => &enm.position,
            Definition::InputExtension(input) => &input.position,
        }
    }
}
