use super::Position;

#[derive(Debug)]
pub struct EnumDefinition {
    pub name: String,
    pub description: Option<String>,
    pub values: Vec<EnumValue>,
    pub position: Position,
}

#[derive(Debug)]
pub struct EnumValue {
    pub name: String,
    pub description: Option<String>,
    pub deprecation_reason: Option<String>,
    pub position: Position,
}

#[derive(Debug)]
pub struct EnumExtension {
    pub name: String,
    pub values: Vec<EnumValue>,
    pub position: Position,
}

impl EnumExtension {
    pub fn iter_values(&self) -> impl Iterator<Item = &EnumValue> {
        self.values.iter()
    }
}

// impl Schema {
//     // Enum
//     pub fn collect_enum_values<'a>(&'a self, def: &'a EnumDefinition) -> impl Iterator<Item = &'a EnumValue> {
//         def.values.iter().chain(
//             self.iter_enum_extensions(&def.name)
//                 .flat_map(|ext| ext.values.iter()),
//         )
//     }

//     fn iter_enum_extensions(&self, name: &str) -> impl Iterator<Item = &EnumExtension> {
//         self.iter_extensions(name).flat_map(|ext| match ext {
//             TypeExtension::EnumExtension(ext) => Some(ext),
//             _ => None,
//         })
//     }
// }
