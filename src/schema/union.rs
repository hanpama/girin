use super::{Position, Schema};

#[derive(Debug)]
pub struct UnionDefinition {
    pub name: String,
    pub description: Option<String>,
    pub types: Vec<String>,
    pub position: Position,
}

#[derive(Debug)]
pub struct UnionExtension {
    pub name: String,
    pub types: Vec<String>,
    pub position: Position,
}

// impl Schema {
//     pub fn collect_possible_types<'a>(
//         &'a self,
//         def: &'a UnionDefinition,
//     ) -> impl Iterator<Item = &'a String> {
//         def.types.iter().chain(
//             self.iter_union_extensions(&def.name)
//                 .flat_map(|ext| ext.types.iter()),
//         )
//     }

//     fn iter_union_extensions(&self, name: &str) -> impl Iterator<Item = &UnionExtension> {
//         self.iter_extensions(name).flat_map(|ext| match ext {
//             TypeExtension::UnionExtension(ext) => Some(ext),
//             _ => None,
//         })
//     }
// }
