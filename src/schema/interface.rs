use super::{Extension, Field, Position, Schema, SourceConfig};

#[derive(Debug)]
pub struct Interface {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<Field>,
    pub interfaces: Vec<String>,
    pub position: Position,
}

impl Interface {
    pub fn collect_source_configs(&self) -> Vec<SourceConfig> {
        self.fields
            .iter()
            .flat_map(|field| field.collect_source_configs())
            .collect()
    }
}

#[derive(Debug)]
pub struct InterfaceTypeExtension {
    pub name: String,
    pub fields: Vec<Field>,
    pub interfaces: Vec<String>,
    pub position: Position,
}

impl Schema {
    pub fn collect_interface_fields<'a>(
        &'a self,
        def: &'a Interface,
    ) -> impl Iterator<Item = &'a Field> {
        def.fields.iter().chain(
            self.iter_interface_extensions(&def.name)
                .flat_map(|ext| ext.fields.iter()),
        )
    }

    pub fn collect_interface_source_configs(&self, def: &Interface) -> Vec<SourceConfig> {
        def.collect_source_configs()
    }

    pub fn collect_interface_interfaces<'a>(
        &'a self,
        def: &'a Interface,
    ) -> impl Iterator<Item = &'a String> {
        def.interfaces.iter().chain(
            self.iter_interface_extensions(&def.name)
                .flat_map(|ext| ext.interfaces.iter()),
        )
    }

    fn iter_interface_extensions(
        &self,
        name: &str,
    ) -> impl Iterator<Item = &InterfaceTypeExtension> {
        self.iter_extensions(name).flat_map(|ext| match ext {
            Extension::InterfaceExtension(ext) => Some(ext),
            _ => None,
        })
    }
}
