use std::collections::{BTreeMap, BTreeSet, HashMap};

use super::{
    definition::Extension, Definition, EnumValue, Field, InputValue, Module, Resolve, Type,
};

#[derive(Debug)]
pub struct Schema {
    query: Option<String>,
    mutation: Option<String>,
    subscription: Option<String>,

    types: BTreeMap<Module, Vec<Type>>,
    def_locs: HashMap<String, Loc>,
    ext_locs: HashMap<String, Vec<Loc>>,
    module_children: HashMap<Module, BTreeSet<String>>,
    type_field_locs: HashMap<(String, String), Loc>,
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
            type_field_locs: HashMap::new(),
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

    pub fn iter_definitions(&self) -> impl Iterator<Item = &Definition> {
        self.types.values().flat_map(|types| {
            types.iter().filter_map(|type_| match type_ {
                Type::Definition(def) => Some(def),
                _ => None,
            })
        })
    }

    pub fn collect_interfaces<'a>(&'a self, name: &'a str) -> Vec<&'a str> {
        let definition_interfaces = match self.get_definition(name) {
            Definition::ObjectDefinition(object) => object.iter_interfaces().collect(),
            Definition::InterfaceDefinition(interface) => interface.iter_interfaces().collect(),
            _ => vec![],
        };

        let extension_interfaces = self.get_extentions(name).flat_map(|ext| match ext {
            Extension::ObjectExtension(object) => object.iter_interfaces().collect(),
            Extension::InterfaceExtension(interface) => interface.iter_interfaces().collect(),
            _ => vec![],
        });

        definition_interfaces
            .into_iter()
            .chain(extension_interfaces)
            .collect()
    }

    pub fn collect_fields<'a>(&'a self, name: &'a str) -> Vec<&'a Field> {
        let definition_fields = match self.get_definition(name) {
            Definition::ObjectDefinition(object) => object.iter_fields().collect(),
            Definition::InterfaceDefinition(interface) => interface.iter_fields().collect(),
            _ => vec![],
        };

        let extension_fields = self.get_extentions(name).flat_map(|ext| match ext {
            Extension::ObjectExtension(object) => object.iter_fields().collect(),
            Extension::InterfaceExtension(interface) => interface.iter_fields().collect(),
            _ => vec![],
        });

        definition_fields
            .into_iter()
            .chain(extension_fields)
            .collect()
    }

    pub fn collect_union_types<'a>(&'a self, name: &'a str) -> Vec<&'a str> {
        let definition_types = self.get_definition(name).as_union().iter_types();

        let extension_types = self
            .get_extentions(name)
            .flat_map(|ext| ext.as_union_ext().iter_types());

        definition_types.chain(extension_types).collect()
    }

    pub fn collect_enum_values<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a EnumValue> {
        let definition_values = self.get_definition(name).as_enum().iter_values();

        let extension_values = self
            .get_extentions(name)
            .flat_map(|ext| ext.as_enum_ext().iter_values());

        definition_values.chain(extension_values)
    }

    pub fn collect_input_fields<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = &'a InputValue> {
        let definition_fields = self.get_definition(name).as_input().iter_fields();

        let extension_fields = self
            .get_extentions(name)
            .flat_map(|ext| ext.as_input_ext().iter_fields());

        definition_fields.chain(extension_fields)
    }

    pub fn get_module_children(&self, module: &Module) -> Option<impl Iterator<Item = &String>> {
        self.module_children
            .get(module)
            .map(|children| children.iter())
    }

    pub fn get_module_types(&self, module: &Module) -> Option<&Vec<Type>> {
        self.types.get(module)
    }

    pub fn resolve_field_resolve<'a>(&'a self, field: &'a Field) -> Option<Resolve<'a>> {
        // println!("resolve_field_resolve: {:?}", field);
        // If the field has a resolve config, return it
        if let Some(conf) = &field.resolve {
            return Some(Resolve::new(conf.sync, field));
        }

        // // Iterate over the interfaces that the field's type implements and
        // // return the first one that has a resolve config for the field
        // for interface in self.collect_interfaces(&field.type_name) {
        //     let key = (interface.to_owned(), field.name.to_owned());
        //     if let Some(loc) = self.type_field_locs.get(&key) {
        //         if let Some(field) = self.get_type(loc).get_field(&field.name) {
        //             println!("found field: {:?}", field);
        //             self.resolve_field_resolve(field);
        //         };
        //     }
        // }

        // If the field's owner is an object type and the field has arguments,
        // create an implicit resolver config and return it
        if let Definition::ObjectDefinition(_) = self.get_definition(&field.type_name) {
            if field.args.len() > 0 {
                return Some(Resolve::new(false, field));
            }
        }

        None
    }

    fn add_type(&mut self, type_: Type) {
        let name = type_.get_name().to_owned();
        let module = type_.get_module().clone();
        let module_types = self.types.entry(module.clone()).or_insert_with(Vec::new);

        for field in type_.iter_fields() {
            let type_field = (name.clone(), field.name.clone());
            let loc = Loc(module.clone(), module_types.len());
            self.type_field_locs.insert(type_field, loc);
        }
        match &type_ {
            Type::Definition(_) => {
                self.def_locs
                    .insert(name.clone(), Loc(module.clone(), module_types.len()));
            }
            Type::Extension(_) => {
                self.ext_locs
                    .entry(name.clone())
                    .or_insert_with(Vec::new)
                    .push(Loc(module.clone(), module_types.len()));
            }
        }
        module_types.push(type_);
        self.index_module(&module);
    }

    fn index_module(&mut self, module: &Module) {
        if module.len() == 0 {
            return;
        }
        let mut module = module.clone();
        let name = module.pop().unwrap();
        self.module_children
            .entry(module.clone())
            .or_insert_with(BTreeSet::new)
            .insert(name);
        self.index_module(&module);
    }

    fn get_definition(&self, name: &str) -> &Definition {
        self.get_type(&self.def_locs[name]).as_definition()
    }

    fn get_type(&self, loc: &Loc) -> &Type {
        &self.types[&loc.0][loc.1]
    }

    fn get_extentions<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &Extension> {
        self.ext_locs
            .get(name)
            .into_iter()
            .flat_map(|locs| locs.iter())
            .map(|loc| self.get_type(loc).as_extension())
    }
}

#[derive(Debug)]
struct Loc(Module, usize);
