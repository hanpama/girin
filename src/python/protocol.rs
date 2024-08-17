use super::{error::Result, naming, sourcecode::SourceCode};
use crate::schema::{
    Definition, Extension, InterfaceDefinition, InterfaceExtension, Module, ObjectDefinition,
    ObjectExtension, ScalarDefinition, Schema, Traversal, Type,
};
use std::{fs::File, path::PathBuf};

pub fn render_builder_config(outdir: &PathBuf, s: &Schema) -> Result<()> {
    render_directory(outdir.join("protocol"), Traversal::new(s))?;
    Ok(())
}

fn render_directory(path: PathBuf, d: Traversal) -> Result<()> {
    if d.has_children() {
        println!("rendering directory: {:?}", path);
        std::fs::create_dir_all(&path)?;
        for child in d.iter_children() {
            let child_path = path.join(child.get_module_name());
            render_directory(child_path, child)?;
        }
        render_config_index(&path, d.clone())?;
    }
    if d.has_types() {
        render_module_config(&path.with_extension("py"), d.clone())?;
    }

    Ok(())
}

fn render_config_index<'a>(parent_path: &PathBuf, d: Traversal) -> Result<()> {
    let filepath = parent_path.join("__init__.py");
    let mut file = File::create(filepath)?;

    let mut src = SourceCode::new();

    src.import_std("typing");

    src.line("class Config(typing.NamedTuple):");

    src.indent();
    for dir in d.iter_children() {
        let name = dir.get_module_name();
        src.import_first(&format!(".{name}"));
        src.line(&format!(
            "{name}: {type}.Config",
            name = name,
            type = name,
        ));
    }
    src.dedent();

    src.write_to(&mut file)?;

    Ok(())
}

fn render_module_config(filepath: &PathBuf, d: Traversal) -> Result<()> {
    println!("rendering module: {:?}", filepath);
    let mut file = File::create(filepath)?;

    let mut src = SourceCode::new();

    src.import_std("typing");

    src.line("class Config(typing.NamedTuple):");
    src.indent();

    let mut pass = true;

    for def in d.iter_types() {
        pass = false;
        match def {
            Type::Definition(inner) => match inner {
                Definition::ObjectDefinition(inner) => {
                    render_object_config(&mut src, inner);
                    src.line("");
                }
                Definition::InterfaceDefinition(inner) => {
                    render_interface_config(&mut src, inner);
                    src.line("");
                }
                Definition::ScalarDefinition(inner) => {
                    render_scalar_config(&mut src, inner);
                    src.line("");
                }
                Definition::InputDefinition(inner) => { /* noop */ }
                Definition::EnumDefinition(inner) => { /* noop */ }
                Definition::UnionDefinition(inner) => { /* noop */ }
            },
            Type::Extension(inner) => match inner {
                Extension::ObjectExtension(inner) => {
                    render_object_ext_config(&mut src, inner);
                    src.line("");
                }
                Extension::InterfaceExtension(inner) => {
                    render_interface_ext_config(&mut src, inner);
                    src.line("");
                }
                Extension::InputExtension(inner) => { /* noop */ }
                Extension::EnumExtension(inner) => { /* noop */ }
                Extension::UnionExtension(inner) => { /* noop */ }
            },
        }
    }

    for def in d.iter_types() {
        match def {
            Type::Definition(inner) => match inner {
                Definition::ObjectDefinition(inner) => {
                    src.line(&format!(
                        "{name}: {type}",
                        name = inner.name,
                        type = naming::config_type(&inner.name),
                    ));
                }
                Definition::InterfaceDefinition(inner) => {
                    src.line(&format!(
                        "{name}: {type}",
                        name = inner.name,
                        type = naming::config_type(&inner.name),
                    ));
                }
                Definition::ScalarDefinition(inner) => {
                    src.line(&format!(
                        "{name}: {type}",
                        name = inner.name,
                        type = naming::config_type(&inner.name),
                    ));
                }
                Definition::InputDefinition(inner) => { /* noop */ }
                Definition::EnumDefinition(inner) => { /* noop */ }
                Definition::UnionDefinition(inner) => { /* noop */ }
            },
            Type::Extension(inner) => match inner {
                Extension::ObjectExtension(inner) => {
                    src.line(&format!(
                        "{name}: {type}",
                        name = inner.name,
                        type = naming::ext_object_resolver_type(&inner),
                    ));
                }
                Extension::InterfaceExtension(inner) => {
                    src.line(&format!(
                        "{name}: {type}",
                        name = inner.name,
                        type = naming::ext_interface_resolver_type(&inner),
                    ));
                }
                Extension::InputExtension(inner) => { /* noop */ }
                Extension::EnumExtension(inner) => { /* noop */ }
                Extension::UnionExtension(inner) => { /* noop */ }
            },
        }
    }

    if pass {
        src.line("pass");
    }
    src.indent();

    src.write_to(&mut file)?;

    Ok(())
}

fn render_object_config(src: &mut SourceCode, def: &ObjectDefinition) {
    src.line(&format!(
        "class {name}(typing.Protocol):",
        name = naming::def_object_resolver_type(def)
    ));
    src.indent();

    let mut pass = true;
    for resolve in def.collect_resolve_configs() {
        pass = false;
        let source_type = import_source(src, &def.module, &def.name);
        src.line(&format!(
            "def {field}(self, obj: {obj_type}, info, **args): ...",
            field = naming::resolver_name(&resolve.name),
            obj_type = source_type,
        ));
    }
    if pass {
        src.line("pass");
    }
    src.dedent();
}

fn render_interface_config(src: &mut SourceCode, def: &InterfaceDefinition) {
    src.line(&format!(
        "class {name}(typing.Protocol):",
        name = naming::def_interface_resolver_type(def)
    ));
    src.indent();

    let mut pass = true;
    for resolve in def.collect_resolve_configs() {
        pass = false;
        let source_type = import_source(src, &def.module, &def.name);
        src.line(&format!(
            "def {field}(self, obj: {obj_type}, info, **args): ...",
            field = naming::resolver_name(&resolve.name),
            obj_type = source_type,
        ));
    }
    if pass {
        src.line("pass");
    }
    src.dedent();
}

fn render_scalar_config(src: &mut SourceCode, def: &ScalarDefinition) {
    src.import_std("typing");
    src.import_third("graphql");
    src.line(&format!(
        "class {name}(typing.Protocol):",
        name = naming::config_type(&def.name)
    ));
    src.indent();

    src.line("def serialize(self, value: typing.Any) -> typing.Any: ...");
    src.line("def parse_value(self, value: typing.Any) -> typing.Any: ...");
    src.line("def parse_literal(self, node: graphql.ValueNode, variables) -> typing.Any: ...");

    src.dedent();
}

fn render_object_ext_config(src: &mut SourceCode, def: &ObjectExtension) {
    src.line(&format!(
        "class {name}(typing.Protocol):",
        name = naming::ext_object_resolver_type(def)
    ));
    src.indent();

    let mut pass = true;
    for resolve in def.collect_resolve_configs() {
        pass = false;
        let source_type = import_source(src, &def.module, &def.name);
        src.line(&format!(
            "def {field}(self, obj: {obj_type}, info, **args): ...",
            field = naming::resolver_name(&resolve.name),
            obj_type = source_type,
        ));
    }
    if pass {
        src.line("pass");
    }
    src.dedent();
}

fn render_interface_ext_config(src: &mut SourceCode, def: &InterfaceExtension) {
    src.line(&format!(
        "class {name}(typing.Protocol):",
        name = naming::ext_interface_resolver_type(def)
    ));
    src.indent();
    let mut pass = true;
    print!("{:?}", def);
    for resolve in def.collect_resolve_configs() {
        print!("res: {:?}", resolve.name);
        pass = false;
        let source_type = import_source(src, &def.module, &def.name);
        src.line(&format!(
            "def {field}(self, obj: {obj_type}, info, **args): ...",
            field = naming::resolver_name(&resolve.name),
            obj_type = source_type,
        ));
    }
    if pass {
        src.line("pass");
    }
    src.dedent();
}

fn import_source(src: &mut SourceCode, m: &Module, def_name: &str) -> String {
    let mut import_path = String::new();
    for _ in 0..m.len() {
        import_path.push_str(".");
    }
    let source_name = naming::source(def_name);
    let import = format!(".{import_path}.source");
    src.import_first(&import);

    format!("source.{source_name}", source_name = source_name)
}
