use std::collections::{BTreeMap, HashMap};

use super::{definition::Extension, Definition, EnumValue, Field, InputValue, Module, Type};

#[derive(Debug)]
pub struct Schema {
    query: Option<String>,
    mutation: Option<String>,
    subscription: Option<String>,

    types: BTreeMap<Module, Vec<Type>>,
    def_locs: HashMap<String, (Module, usize)>,
    ext_locs: HashMap<String, Vec<(Module, usize)>>,
    module_children: HashMap<Module, Vec<String>>,
}

impl Schema {
    pub fn new() -> Self {
        Self {
            query: None,
            mutation: None,
            subscription: None,
            types: BTreeMap::new(),
            def_locs: HashMap::new(),
            ext_locs: HashMap::new(),
            module_children: HashMap::new(),
        }
    }
    pub fn get_query(&self) -> Option<&str> {
        self.query.as_deref()
    }
    pub fn set_query(&mut self, query: Option<String>) {
        self.query = query;
    }
    pub fn get_mutation(&self) -> Option<&str> {
        self.mutation.as_deref()
    }
    pub fn set_mutation(&mut self, mutation: Option<String>) {
        self.mutation = mutation;
    }
    pub fn get_subscription(&self) -> Option<&str> {
        self.subscription.as_deref()
    }
    pub fn set_subscription(&mut self, subscription: Option<String>) {
        self.subscription = subscription;
    }
    pub fn add_definition(&mut self, definition: Definition) {
        self.add_type(Type::Definition(definition));
    }
    pub fn add_extension(&mut self, extension: Extension) {
        self.add_type(Type::Extension(extension));
    }

    fn add_type(&mut self, type_: Type) {
        let name = type_.get_name().to_owned();
        let module = type_.get_module().clone();
        let module_types = self.types.entry(module.clone()).or_insert_with(Vec::new);
        let loc = (module.clone(), module_types.len());

        match type_ {
            Type::Definition(_) => {
                self.def_locs.insert(name, loc);
            }
            Type::Extension(_) => {
                self.ext_locs.entry(name).or_insert_with(Vec::new).push(loc);
            }
        }
        module_types.push(type_);
        self.index_module(&module);
    }

    fn index_module(&mut self, module: &Module) {
        if self.module_children.contains_key(module) || module.len() == 0 {
            return;
        }
        let mut module = module.clone();
        let name = module.pop().unwrap();
        self.module_children
            .entry(module.clone())
            .or_insert_with(Vec::new)
            .push(name);
        self.index_module(&module);
    }

    pub fn iter_definitions(&self) -> impl Iterator<Item = &Definition> {
        self.types.values().flat_map(|types| {
            types.iter().filter_map(|type_| match type_ {
                Type::Definition(def) => Some(def),
                _ => None,
            })
        })
    }

    pub fn get_definition(&self, name: &str) -> &Definition {
        let (module, idx) = self.def_locs.get(name).unwrap();
        self.types[module][*idx].as_definition()
    }

    pub fn collect_extentions<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &Extension> {
        self.ext_locs
            .get(name)
            .into_iter()
            .flat_map(|locs| locs.iter())
            .map(|(module, idx)| self.types[module][*idx].as_extension())
    }

    pub fn collect_object_interfaces<'a>(&'a self, name: &'a str) -> Vec<&'a str> {
        let definition_interfaces = self.get_definition(name).as_object().iter_interfaces();

        let extension_interfaces = self
            .collect_extentions(name)
            .flat_map(|ext| ext.as_object_ext().iter_interfaces());

        definition_interfaces.chain(extension_interfaces).collect()
    }

    pub fn collect_object_fields<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Field> {
        let definition_fields = self.get_definition(name).as_object().iter_fields();

        let extension_fields = self
            .collect_extentions(name)
            .flat_map(|ext| ext.as_object_ext().iter_fields());

        definition_fields.chain(extension_fields)
    }

    pub fn collect_interface_interfaces<'a>(&'a self, name: &'a str) -> Vec<&'a str> {
        let definition_interfaces = self.get_definition(name).as_interface().iter_interfaces();

        let extension_interfaces = self
            .collect_extentions(name)
            .flat_map(|ext| ext.as_interface_ext().iter_interfaces());

        definition_interfaces.chain(extension_interfaces).collect()
    }

    pub fn collect_interface_fields<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = &'a Field> {
        let definition_fields = self.get_definition(name).as_interface().iter_fields();

        let extension_fields = self
            .collect_extentions(name)
            .flat_map(|ext| ext.as_interface_ext().iter_fields());

        definition_fields.chain(extension_fields)
    }

    pub fn collect_union_types<'a>(&'a self, name: &'a str) -> Vec<&'a str> {
        let definition_types = self.get_definition(name).as_union().iter_types();

        let extension_types = self
            .collect_extentions(name)
            .flat_map(|ext| ext.as_union_ext().iter_types());

        definition_types.chain(extension_types).collect()
    }

    pub fn collect_enum_values<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a EnumValue> {
        let definition_values = self.get_definition(name).as_enum().iter_values();

        let extension_values = self
            .collect_extentions(name)
            .flat_map(|ext| ext.as_enum_ext().iter_values());

        definition_values.chain(extension_values)
    }

    pub fn collect_input_fields<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = &'a InputValue> {
        let definition_fields = self.get_definition(name).as_input().iter_fields();

        let extension_fields = self
            .collect_extentions(name)
            .flat_map(|ext| ext.as_input_ext().iter_fields());

        definition_fields.chain(extension_fields)
    }
}
