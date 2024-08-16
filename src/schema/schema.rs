use super::{Definition, Directory, EnumValue, ExtensionRef, Field, InputValue};

#[derive(Debug)]
pub struct Schema {
    pub root_dir: Directory,
}

impl Schema {
    pub fn new(root_dir: Directory) -> Self {
        Self { root_dir }
    }

    pub fn iter_definitions(&self) -> impl Iterator<Item = &Definition> {
        self.root_dir.iter_definitions()
    }

    pub fn get_definition(&self, name: &str) -> &Definition {
        self.root_dir.get_definition(name)
    }

    pub fn collect_extentions<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = ExtensionRef<'a>> {
        self.root_dir.resolve_extensions(name)
    }

    pub fn collect_object_interfaces<'a>(&'a self, name: &'a str) -> Vec<&'a str> {
        let definition_interfaces = self
            .get_definition(name)
            .as_object()
            .unwrap()
            .iter_interfaces();

        let extension_interfaces = self
            .collect_extentions(name)
            .flat_map(|ext| ext.extension.as_object().unwrap().iter_interfaces());

        definition_interfaces.chain(extension_interfaces).collect()
    }

    pub fn collect_object_fields<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Field> {
        let definition_fields = self.get_definition(name).as_object().unwrap().iter_fields();

        let extension_fields = self
            .collect_extentions(name)
            .flat_map(|ext| ext.extension.as_object().unwrap().iter_fields());

        definition_fields.chain(extension_fields)
    }

    pub fn collect_interface_interfaces<'a>(&'a self, name: &'a str) -> Vec<&'a str> {
        let definition_interfaces = self
            .get_definition(name)
            .as_interface()
            .unwrap()
            .iter_interfaces();

        let extension_interfaces = self
            .collect_extentions(name)
            .flat_map(|ext| ext.extension.as_interface().unwrap().iter_interfaces());

        definition_interfaces.chain(extension_interfaces).collect()
    }

    pub fn collect_interface_fields<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = &'a Field> {
        let definition_fields = self
            .get_definition(name)
            .as_interface()
            .unwrap()
            .iter_fields();

        let extension_fields = self
            .collect_extentions(name)
            .flat_map(|ext| ext.extension.as_interface().unwrap().iter_fields());

        definition_fields.chain(extension_fields)
    }

    pub fn collect_union_types<'a>(&'a self, name: &'a str) -> Vec<&'a str> {
        let definition_types = self.get_definition(name).as_union().unwrap().iter_types();

        let extension_types = self
            .collect_extentions(name)
            .flat_map(|ext| ext.extension.as_union().unwrap().iter_types());

        definition_types.chain(extension_types).collect()
    }

    pub fn collect_enum_values<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a EnumValue> {
        let definition_values = self.get_definition(name).as_enum().unwrap().iter_values();

        let extension_values = self
            .collect_extentions(name)
            .flat_map(|ext| ext.extension.as_enum().unwrap().iter_values());

        definition_values.chain(extension_values)
    }

    pub fn collect_input_fields<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = &'a InputValue> {
        let definition_fields = self.get_definition(name).as_input().unwrap().iter_fields();

        let extension_fields = self
            .collect_extentions(name)
            .flat_map(|ext| ext.extension.as_input().unwrap().iter_fields());

        definition_fields.chain(extension_fields)
    }
}
