use super::{Module, Schema, Type};

#[derive(Clone)]
pub struct Traversal<'a> {
    pub schema: &'a Schema,
    pub module: Module,
}

impl Traversal<'_> {
    pub fn new(schema: &Schema) -> Traversal {
        let module = vec![];
        Traversal { schema, module }
    }

    fn new_child(&self, name: &str) -> Traversal {
        let mut module = self.module.clone();
        module.push(name.to_owned());
        Traversal {
            schema: self.schema,
            module,
        }
    }

    pub fn get_module_name(&self) -> &str {
        self.module.last().unwrap()
    }

    pub fn has_children(&self) -> bool {
        self.schema.get_module_children(&self.module).is_some()
    }

    pub fn iter_children(&self) -> impl Iterator<Item = Traversal> {
        self.schema
            .get_module_children(&self.module)
            .unwrap()
            .map(|name| self.new_child(name))
    }

    pub fn has_types(&self) -> bool {
        self.schema.get_module_types(&self.module).is_some()
    }

    pub fn iter_types(&self) -> impl Iterator<Item = &Type> {
        self.schema.get_module_types(&self.module).unwrap().iter()
    }
}
