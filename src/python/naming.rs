use crate::definitions::{
    Enum, Input, Interface, InterfaceTypeExtension,
    Module, Object, ObjectExtension, Scalar, Submodule,
};

pub fn module_config_type(md: &Module) -> String {
    format!("{}", md.name)
}

pub fn module_config_field_name(md: &Module) -> String {
    format!("_{}", md.name)
}

pub fn submodule_config_type(smd: &Submodule) -> String {
    format!("{}", smd.name)
}

pub fn submodule_config_field_name(smd: &Submodule) -> String {
    format!("_{}", smd.name)
}

pub fn object_def_resolver_type(def: &Object) -> String {
    format!("{}Resolver", def.name)
}
pub fn interface_def_resolver_type(def: &Interface) -> String {
    format!("{}Resolver", def.name)
}
pub fn object_ext_resolver_type(def: &ObjectExtension) -> String {
    format!("{}Resolver", def.name)
}
pub fn interface_ext_resolver_type(def: &InterfaceTypeExtension) -> String {
    format!("{}Resolver", def.name)
}

pub fn object_source(def: &Object) -> String {
    format!("{}Source", def.name)
}

pub fn interface_source(def: &Interface) -> String {
    format!("{}Source", def.name)
}

pub fn input_source(def: &Input) -> String {
    format!("{}Source", def.name)
}

pub fn scalar_source(def: &Scalar) -> String {
    format!("{}Source", def.name)
}

pub fn enum_source(def: &Enum) -> String {
    format!("{}Source", def.name)
}

pub fn source<S: Into<String>>(def_name: S) -> String {
    format!("{}Source", def_name.into())
}

pub fn object_type_instance(def: &Object) -> String {
    format!("{}", def.name)
}
pub fn input_type_instance(def: &Input) -> String {
    format!("{}", def.name)
}
pub fn type_instance(def_name: &str) -> String {
    format!("{}", def_name)
}
pub fn interface_type_instance(def: &Interface) -> String {
    format!("{}", def.name)
}
