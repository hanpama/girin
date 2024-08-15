use super::{
    EnumDefinition, EnumExtension, InputDefinition, InputExtension, InterfaceDefinition,
    InterfaceExtension, ObjectDefinition, ObjectExtension, ScalarDefinition, UnionDefinition,
    UnionExtension,
};

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
}

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
}
