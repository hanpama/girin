use super::{Extension, InputValue, Position, Schema, SourceConfig};

#[derive(Debug)]
pub struct Input {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<InputValue>,
    pub position: Position,
}

impl Input {
    pub fn collect_source_configs(&self) -> Vec<SourceConfig> {
        self.fields
            .iter()
            .map(|field| SourceConfig {
                name: field.name.clone(),
                type_: field.field_type.clone(),
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct InputExtension {
    pub name: String,
    pub fields: Vec<InputValue>,
    pub position: Position,
}

impl Schema {
    pub fn collect_input_fields<'a>(
        &'a self,
        def: &'a Input,
    ) -> impl Iterator<Item = &'a InputValue> {
        def.fields.iter().chain(
            self.iter_input_extensions(&def.name)
                .flat_map(|ext| ext.fields.iter()),
        )
    }

    pub fn collect_input_source_configs(&self, def: &Input) -> Vec<SourceConfig> {
        def.collect_source_configs()
    }

    fn iter_input_extensions(&self, name: &str) -> impl Iterator<Item = &InputExtension> {
        self.iter_extensions(name).flat_map(|ext| match ext {
            Extension::InputExtension(ext) => Some(ext),
            _ => None,
        })
    }
}
