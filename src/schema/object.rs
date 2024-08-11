use super::{Extension, Field, Position, Schema, SourceConfig};

#[derive(Debug)]
pub struct Object {
    pub fields: Vec<Field>,
    pub name: String,
    pub description: Option<String>,
    pub interfaces: Vec<String>,
    pub position: Position,
}

impl Object {
    pub fn collect_source_configs(&self) -> Vec<SourceConfig> {
        self.fields
            .iter()
            .flat_map(|field| field.collect_source_configs())
            .collect()
    }
}

#[derive(Debug)]
pub struct ObjectExtension {
    pub name: String,
    pub interfaces: Vec<String>,
    pub fields: Vec<Field>,
    pub position: Position,
}

impl ObjectExtension {
    pub fn collect_source_configs(&self) -> Vec<SourceConfig> {
        self.fields
            .iter()
            .flat_map(|field| field.collect_source_configs())
            .collect()
    }
}

impl Schema {
    pub fn collect_object_source_configs(&self, def: &Object) -> Vec<SourceConfig> {
        let mut cfs = def.collect_source_configs();
        for ext in self.iter_object_extensions(&def.name) {
            cfs.extend(ext.collect_source_configs());
        }
        cfs
    }

    pub fn collect_object_fields<'a>(&'a self, def: &'a Object) -> impl Iterator<Item = &'a Field> {
        def.fields.iter().chain(
            self.iter_object_extensions(&def.name)
                .flat_map(|ext| ext.fields.iter()),
        )
    }

    pub fn collect_object_interfaces<'a>(
        &'a self,
        def: &'a Object,
    ) -> impl Iterator<Item = &'a String> {
        def.interfaces.iter().chain(
            self.iter_object_extensions(&def.name)
                .flat_map(|ext| ext.interfaces.iter()),
        )
    }

    fn iter_object_extensions(&self, name: &str) -> impl Iterator<Item = &ObjectExtension> {
        self.iter_extensions(name).flat_map(|ext| match ext {
            Extension::ObjectExtension(ext) => Some(ext),
            _ => None,
        })
    }
}
