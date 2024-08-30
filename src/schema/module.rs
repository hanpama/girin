use std::path::PathBuf;

use super::{Definition, Schema};

#[derive(Clone)]
pub struct ModuleRef<'a> {
    pub schema: &'a Schema,
    pub path: &'a PathBuf,
}

impl ModuleRef<'_> {
    pub fn new(schema: &Schema) -> ModuleRef {
        ModuleRef {
            schema,
            path: &schema.get_root_dir(),
        }
    }

    pub fn get_name(&self) -> &str {
        self.path.file_name().unwrap().to_str().unwrap()
    }

    pub fn get_breadcrumbs(&self) -> Vec<&str> {
        self.path
            .strip_prefix(&self.schema.get_root_dir())
            .iter()
            .map(|s| s.to_str().unwrap())
            .collect()
    }

    pub fn get_depth(&self) -> usize {
        self.path
            .strip_prefix(&self.schema.get_root_dir())
            .unwrap()
            .components()
            .count()
    }

    pub fn has_children(&self) -> bool {
        self.schema.get_module_children(&self.path).is_some()
    }

    pub fn iter_children(&self) -> impl Iterator<Item = ModuleRef> {
        self.schema
            .get_module_children(self.path)
            .unwrap()
            .map(|child| ModuleRef {
                schema: self.schema,
                path: child,
            })
    }

    pub fn has_definition(&self) -> bool {
        self.schema.get_module_definitions(&self.path).is_some()
    }

    pub fn iter_definitions(&self) -> impl Iterator<Item = &Definition> {
        self.schema
            .get_module_definitions(&self.path)
            .unwrap()
            .iter()
    }
}
