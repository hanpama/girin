use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    path::PathBuf,
};

use super::{Definition, EnumValue, Field, InputValue, ModuleRef, Resolve};

#[derive(Debug)]
pub struct Project {
    root_dir: PathBuf,
    definitions: BTreeMap<PathBuf, Vec<Definition>>,
    schema_loc: Option<Loc>,
    def_locs: HashMap<String, Loc>,
    ext_locs: HashMap<String, Vec<Loc>>,
    module_children: HashMap<PathBuf, BTreeSet<PathBuf>>,
    type_field_locs: HashMap<(String, String), Loc>,
}

impl Project {
    pub fn new(root_dir: PathBuf) -> Self {
        Self {
            // query: None,
            // mutation: None,
            // subscription: None,
            root_dir,
            definitions: BTreeMap::new(),
            schema_loc: None,
            def_locs: HashMap::new(),
            ext_locs: HashMap::new(),
            module_children: HashMap::new(),
            type_field_locs: HashMap::new(),
        }
    }
    pub fn get_query(&self) -> Option<&str> {
        if let Some(loc) = &self.schema_loc {
            return self.get_type(loc).as_schema().query.as_deref();
        }
        self.def_locs.contains_key("Query").then(|| "Query")
    }

    pub fn get_mutation(&self) -> Option<&str> {
        if let Some(loc) = &self.schema_loc {
            return self.get_type(loc).as_schema().mutation.as_deref();
        }
        self.def_locs.contains_key("Mutation").then(|| "Mutation")
    }

    pub fn get_subscription(&self) -> Option<&str> {
        if let Some(loc) = &self.schema_loc {
            return self.get_type(loc).as_schema().subscription.as_deref();
        }
        self.def_locs
            .contains_key("Subscription")
            .then(|| "Subscription")
    }

    pub fn get_root_dir(&self) -> &PathBuf {
        &self.root_dir
    }

    pub fn iter_type_definitions(&self) -> impl Iterator<Item = &Definition> {
        self.definitions.values().flat_map(|types| {
            types.iter().filter_map(|def| match def {
                Definition::ObjectDefinition(_) => Some(def),
                Definition::EnumDefinition(_) => Some(def),
                Definition::InputDefinition(_) => Some(def),
                Definition::UnionDefinition(_) => Some(def),
                Definition::ScalarDefinition(_) => Some(def),
                Definition::InterfaceDefinition(_) => Some(def),
                _ => None,
            })
        })
    }

    pub fn collect_interfaces<'a>(&'a self, name: &'a str) -> Vec<&'a str> {
        let definition_interfaces = match self.get_type_definition(name) {
            Definition::ObjectDefinition(object) => object.iter_interfaces().collect(),
            Definition::InterfaceDefinition(interface) => interface.iter_interfaces().collect(),
            _ => vec![],
        };

        let extension_interfaces = self.get_type_extentions(name).flat_map(|ext| match ext {
            Definition::ObjectExtension(object) => object.iter_interfaces().collect(),
            Definition::InterfaceExtension(interface) => interface.iter_interfaces().collect(),
            _ => vec![],
        });

        definition_interfaces
            .into_iter()
            .chain(extension_interfaces)
            .collect()
    }

    pub fn collect_fields<'a>(&'a self, name: &'a str) -> Vec<&'a Field> {
        let definition_fields = match self.get_type_definition(name) {
            Definition::ObjectDefinition(object) => object.iter_fields().collect(),
            Definition::InterfaceDefinition(interface) => interface.iter_fields().collect(),
            _ => vec![],
        };

        let extension_fields = self.get_type_extentions(name).flat_map(|ext| match ext {
            Definition::ObjectExtension(object) => object.iter_fields().collect(),
            Definition::InterfaceExtension(interface) => interface.iter_fields().collect(),
            _ => vec![],
        });

        definition_fields
            .into_iter()
            .chain(extension_fields)
            .collect()
    }

    pub fn collect_union_types<'a>(&'a self, name: &'a str) -> Vec<&'a str> {
        let definition_types = self.get_type_definition(name).as_union().iter_types();

        let extension_types = self
            .get_type_extentions(name)
            .flat_map(|ext| ext.as_union_ext().iter_types());

        definition_types.chain(extension_types).collect()
    }

    pub fn collect_enum_values<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a EnumValue> {
        let definition_values = self.get_type_definition(name).as_enum().iter_values();

        let extension_values = self
            .get_type_extentions(name)
            .flat_map(|ext| ext.as_enum_ext().iter_values());

        definition_values.chain(extension_values)
    }

    pub fn collect_input_fields<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = &'a InputValue> {
        let definition_fields = self.get_type_definition(name).as_input().iter_fields();

        let extension_fields = self
            .get_type_extentions(name)
            .flat_map(|ext| ext.as_input_ext().iter_fields());

        definition_fields.chain(extension_fields)
    }

    pub fn get_module_children(&self, file: &PathBuf) -> Option<impl Iterator<Item = &PathBuf>> {
        self.module_children
            .get(file)
            .map(|children| children.iter())
    }

    pub fn get_module_definitions(&self, file: &PathBuf) -> Option<&Vec<Definition>> {
        self.definitions.get(file)
    }

    pub fn resolve_field_resolve<'a>(&'a self, field: &'a Field) -> Option<Resolve<'a>> {
        if let Some(conf) = &field.resolve {
            return Some(Resolve::new(conf.sync, field));
        }
        if field.args.len() > 0 {
            return Some(Resolve::new(false, field));
        }

        None
    }

    pub fn add_definition(&mut self, type_: Definition) {
        let file = type_.get_position().file.clone();

        let module_types = self
            .definitions
            .entry(file.clone())
            .or_insert_with(Vec::new);

        for field in type_.iter_fields() {
            let name = type_.get_definition_name().unwrap().to_owned();

            let type_field = (name, field.name.clone());
            let loc = Loc(file.clone(), module_types.len());
            self.type_field_locs.insert(type_field, loc);
        }
        if type_.is_schema_definition() {
            self.schema_loc = Some(Loc(file.clone(), module_types.len()));
        }
        if type_.is_type_definition() {
            let name = type_.get_definition_name().unwrap().to_owned();
            self.def_locs
                .insert(name, Loc(file.clone(), module_types.len()));
        }
        if type_.is_type_extension() {
            let name = type_.get_definition_name().unwrap().to_owned();
            self.ext_locs
                .entry(name)
                .or_insert_with(Vec::new)
                .push(Loc(file.clone(), module_types.len()));
        }

        module_types.push(type_);
        self.index_module(&file);
    }

    pub fn get_module_ref<'a>(&'a self, path: &'a PathBuf) -> ModuleRef<'a> {
        ModuleRef { schema: self, path }
    }

    fn index_module(&mut self, file: &PathBuf) {
        let is_under_root = file
            .strip_prefix(&self.root_dir)
            .and_then(|stripped| Ok(!stripped.as_os_str().is_empty()))
            .unwrap_or(false);

        if !is_under_root {
            return;
        }

        let parent = file.parent().unwrap().to_path_buf();
        self.module_children
            .entry(parent.clone())
            .or_insert_with(BTreeSet::new)
            .insert(file.to_path_buf());

        self.index_module(&parent);
    }

    fn get_type_definition(&self, name: &str) -> &Definition {
        self.get_type(&self.def_locs[name])
    }

    fn get_type(&self, loc: &Loc) -> &Definition {
        &self.definitions[&loc.0][loc.1]
    }

    fn get_type_extentions<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &Definition> {
        self.ext_locs
            .get(name)
            .into_iter()
            .flat_map(|locs| locs.iter())
            .map(|loc| self.get_type(loc))
    }
}

#[derive(Debug)]
struct Loc(PathBuf, usize);
