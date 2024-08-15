use super::{error::Result, naming, sourcecode::SourceCode};
use crate::schema::{
    Definition, Directory, InterfaceDefinition, InterfaceExtension, Module, ObjectDefinition,
    ObjectExtension, Schema,
};
use std::{fs::File, path::PathBuf};

pub fn render_builder_config(outdir: &PathBuf, s: &Schema) -> Result<()> {
    let root_dir = outdir.join("protocol");
    render_directory(&root_dir, s, &s.root_dir)?;
    Ok(())
}

fn render_directory(parent_dir: &PathBuf, s: &Schema, d: &Directory) -> Result<()> {
    let dir_path = parent_dir.join(d.get_name());
    std::fs::create_dir_all(&dir_path)?;

    for dir in d.iter_directories() {
        render_directory(parent_dir, s, dir)?;
    }
    for md in d.iter_modules() {
        render_module_config(parent_dir, s, md)?;
    }

    render_config_index(&dir_path, d)?;

    Ok(())
}

fn render_config_index<'a>(dir_path: &PathBuf, d: &Directory) -> Result<()> {
    let filepath = dir_path.join("__init__.py");
    let mut file = File::create(filepath)?;

    let mut src = SourceCode::new();

    src.import_third("typing");

    src.line("class Config(typing.NamedTuple):");
    src.indent();
    for dir in d.iter_directories() {
        let name = dir.get_name();
        src.import_first(&format!(".{name}"));
        src.line(&format!(
            "{name}: {type}.Config",
            name = name,
            type = name,
        ));
    }
    for md in d.iter_modules() {
        let name = md.get_name();
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

fn render_module_config(parent_dir: &PathBuf, s: &Schema, m: &Module) -> Result<()> {
    let filepath = parent_dir.join(m.path.with_extension("py"));
    let mut file = File::create(filepath)?;

    let mut src = SourceCode::new();

    src.import_third("typing");

    src.line("class Config(typing.NamedTuple):");
    src.indent();

    let mut pass = true;

    for def in m.iter_definitions() {
        pass = false;
        match def {
            Definition::ObjectDefinition(inner) => {
                render_object_def_resolver(&mut src, s, m, inner);
                src.line("");
            }
            Definition::InterfaceDefinition(inner) => {
                render_interface_def_resolver(&mut src, s, m, inner);
                src.line("");
            }
            Definition::ScalarDefinition(inner) => {
                // render_scalar_def_resolver(src, s, inner);
            }
            Definition::InputDefinition(inner) => { /* noop */ }
            Definition::EnumDefinition(inner) => { /* noop */ }
            Definition::UnionDefinition(inner) => { /* noop */ }

            Definition::ObjectExtension(inner) => {
                render_object_ext_resolver(&mut src, s, m, inner);
                src.line("");
            }
            Definition::InterfaceExtension(inner) => {
                render_interface_ext_resolver(&mut src, s, m, inner);
                src.line("");
            }
            Definition::InputExtension(inner) => { /* noop */ }
            Definition::EnumExtension(inner) => { /* noop */ }
            Definition::UnionExtension(inner) => { /* noop */ }
            _ => {}
        }
    }

    for def in m.iter_definitions() {
        match def {
            Definition::ObjectDefinition(inner) => {
                src.line(&format!(
                    "{name}: {type}",
                    name = inner.name,
                    type = naming::def_object_resolver_type(&inner),
                ));
            }
            Definition::InterfaceDefinition(inner) => {
                src.line(&format!(
                    "{name}: {type}",
                    name = inner.name,
                    type = naming::def_interface_resolver_type(&inner),
                ));
            }
            Definition::ScalarDefinition(inner) => {
                // render_scalar_def_resolver(src, s, inner);
            }
            Definition::InputDefinition(inner) => { /* noop */ }
            Definition::EnumDefinition(inner) => { /* noop */ }
            Definition::UnionDefinition(inner) => { /* noop */ }

            Definition::ObjectExtension(inner) => {
                src.line(&format!(
                    "{name}: {type}",
                    name = inner.name,
                    type = naming::ext_object_resolver_type(&inner),
                ));
            }
            Definition::InterfaceExtension(inner) => {
                src.line(&format!(
                    "{name}: {type}",
                    name = inner.name,
                    type = naming::ext_interface_resolver_type(&inner),
                ));
            }
            Definition::InputExtension(inner) => { /* noop */ }
            Definition::EnumExtension(inner) => { /* noop */ }
            Definition::UnionExtension(inner) => { /* noop */ }

            _ => {}
        }
    }

    if pass {
        src.line("pass");
    }
    src.indent();

    src.write_to(&mut file)?;

    Ok(())
}

fn render_object_def_resolver(
    src: &mut SourceCode,
    s: &Schema,
    m: &Module,
    def: &ObjectDefinition,
) {
    src.line(&format!(
        "class {name}(typing.Protocol):",
        name = naming::def_object_resolver_type(def)
    ));
    src.indent();

    let mut pass = true;
    for resolve in def.collect_resolve_configs() {
        pass = false;
        let source_type = import_source(src, m, &def.name);
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

fn render_interface_def_resolver(
    src: &mut SourceCode,
    s: &Schema,
    m: &Module,
    def: &InterfaceDefinition,
) {
    src.line(&format!(
        "class {name}(typing.Protocol):",
        name = naming::def_interface_resolver_type(def)
    ));
    src.indent();

    let mut pass = true;
    for resolve in def.collect_resolve_configs() {
        pass = false;
        let source_type = import_source(src, m, &def.name);
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

fn render_object_ext_resolver(src: &mut SourceCode, s: &Schema, m: &Module, def: &ObjectExtension) {
    src.line(&format!(
        "class {name}(typing.Protocol):",
        name = naming::ext_object_resolver_type(def)
    ));
    src.indent();

    let mut pass = true;
    for resolve in def.collect_resolve_configs() {
        pass = false;
        let source_type = import_source(src, m, &def.name);
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

fn render_interface_ext_resolver(
    src: &mut SourceCode,
    s: &Schema,
    m: &Module,
    def: &InterfaceExtension,
) {
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
        let source_type = import_source(src, m, &def.name);
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
    for _ in 0..m.path.components().count() {
        import_path.push_str(".");
    }
    let source_name = naming::source(def_name);
    let import = format!(".{import_path}.source");
    src.import_first(&import);

    format!("source.{source_name}", source_name = source_name)
}
