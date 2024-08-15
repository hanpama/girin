use super::{
    EnumDefinition, EnumExtension, InputDefinition, InputExtension, InterfaceDefinition,
    InterfaceTypeExtension, ObjectDefinition, ObjectExtension, ScalarDefinition, UnionDefinition,
    UnionExtension,
};

pub enum Definition {
    ScalarDefinition(ScalarDefinition),
    ObjectDefinition(ObjectDefinition),
    InterfaceDefinition(InterfaceDefinition),
    UnionDefinition(UnionDefinition),
    EnumDefinition(EnumDefinition),
    InputDefinition(InputDefinition),
    ObjectExtension(ObjectExtension),
    InterfaceExtension(InterfaceTypeExtension),
    UnionExtension(UnionExtension),
    EnumExtension(EnumExtension),
    InputExtension(InputExtension),
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
            Definition::ObjectExtension(ext) => &ext.name,
            Definition::InterfaceExtension(ext) => &ext.name,
            Definition::UnionExtension(ext) => &ext.name,
            Definition::EnumExtension(ext) => &ext.name,
            Definition::InputExtension(ext) => &ext.name,
        }
    }
}

impl Into<ObjectDefinition> for Definition {
    fn into(self) -> ObjectDefinition {
        match self {
            Definition::ObjectDefinition(inner) => inner,
            _ => panic!("Expected ObjectDefinition"),
        }
    }
}
impl Into<ObjectExtension> for Definition {
    fn into(self) -> ObjectExtension {
        match self {
            Definition::ObjectExtension(inner) => inner,
            _ => panic!("Expected ObjectExtension"),
        }
    }
}