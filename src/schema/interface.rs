use super::{field::ResolveOption, Field, Position, Schema, SourceConfig};

#[derive(Debug)]
pub struct InterfaceDefinition {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<Field>,
    pub interfaces: Vec<String>,
    pub position: Position,
}

impl InterfaceDefinition {
    pub fn collect_source_configs(&self) -> Vec<SourceConfig> {
        self.fields
            .iter()
            .flat_map(|field| field.collect_source_configs())
            .collect()
    }

    pub fn collect_resolve_configs(&self) -> Vec<ResolveOption> {
        self.fields
            .iter()
            .map(|field| field.get_resolve_option())
            .filter_map(|opt| opt)
            .collect()
    }
}

#[derive(Debug)]
pub struct InterfaceExtension {
    pub name: String,
    pub fields: Vec<Field>,
    pub interfaces: Vec<String>,
    pub position: Position,
}

impl InterfaceExtension {
    pub fn iter_interfaces(&self) -> impl Iterator<Item = &str> {
        self.interfaces.iter().map(|s| s.as_str())
    }

    pub fn iter_fields(&self) -> impl Iterator<Item = &Field> {
        self.fields.iter()
    }

    pub fn collect_source_configs(&self) -> Vec<SourceConfig> {
        self.fields
            .iter()
            .flat_map(|field| field.collect_source_configs())
            .collect()
    }

    pub fn collect_resolve_configs(&self) -> Vec<ResolveOption> {
        self.fields
            .iter()
            .map(|field| field.get_resolve_option())
            .filter_map(|opt| opt)
            .collect()
    }
}

// impl Schema {
//     pub fn collect_interface_fields<'a>(
//         &'a self,
//         def: &'a InterfaceDefinition,
//     ) -> impl Iterator<Item = &'a Field> {
//         def.fields.iter().chain(
//             self.iter_interface_extensions(&def.name)
//                 .flat_map(|ext| ext.fields.iter()),
//         )
//     }

//     pub fn collect_interface_source_configs(&self, def: &InterfaceDefinition) -> Vec<SourceConfig> {
//         def.collect_source_configs()
//     }

//     pub fn collect_interface_interfaces<'a>(
//         &'a self,
//         def: &'a InterfaceDefinition,
//     ) -> impl Iterator<Item = &'a String> {
//         def.interfaces.iter().chain(
//             self.iter_interface_extensions(&def.name)
//                 .flat_map(|ext| ext.interfaces.iter()),
//         )
//     }

//     fn iter_interface_extensions(
//         &self,
//         name: &str,
//     ) -> impl Iterator<Item = &InterfaceTypeExtension> {
//         self.iter_extensions(name).flat_map(|ext| match ext {
//             TypeExtension::InterfaceExtension(ext) => Some(ext),
//             _ => None,
//         })
//     }
// }
