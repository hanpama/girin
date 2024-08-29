use std::path::PathBuf;

use super::{Definition, Schema};

#[derive(Clone)]
pub struct Traversal<'a> {
    pub schema: &'a Schema,
    pub module: &'a PathBuf,
}

impl Traversal<'_> {
    pub fn new(schema: &Schema) -> Traversal {
        Traversal {
            schema,
            module: &schema.root_dir,
        }
    }

    pub fn get_module_name(&self) -> &str {
        self.module.file_name().unwrap().to_str().unwrap()
    }

    pub fn has_children(&self) -> bool {
        self.schema.get_module_children(&self.module).is_some()
    }

    pub fn iter_children(&self) -> impl Iterator<Item = Traversal> {
        self.schema
            .get_module_children(self.module)
            .unwrap()
            .map(|child| Traversal {
                schema: self.schema,
                module: child,
            })
    }

    pub fn has_definition(&self) -> bool {
        self.schema.get_module_definitions(&self.module).is_some()
    }

    pub fn iter_definitions(&self) -> impl Iterator<Item = &Definition> {
        self.schema
            .get_module_definitions(&self.module)
            .unwrap()
            .iter()
    }
}
