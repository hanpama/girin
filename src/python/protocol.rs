use super::{error::Result, naming, sourcecode::SourceCode};
use crate::schema::{
    Definition, Directory, Interface, InterfaceTypeExtension, Module, Object, ObjectExtension,
    Schema, TypeDefinition, TypeExtension,
};
use std::{fs::File, path::PathBuf};

pub fn render_builder_config(outdir: &PathBuf, s: &Schema) -> Result<()> {
    let root_dir = outdir.join("protocol");

    for dir in s.iter_root_directories() {
        render_directory(&root_dir, s, dir)?;
    }
    for md in s.iter_root_modules() {
        render_module_config(&root_dir, s, md)?;
    }

    render_config_index(&root_dir, s.iter_root_directories(), s.iter_root_modules())?;

    Ok(())
}

fn render_directory(basedir: &PathBuf, s: &Schema, d: &Directory) -> Result<()> {
    let dir_path = basedir.join(&d.path);
    std::fs::create_dir_all(&dir_path)?;

    for dir in s.iter_child_directories(d) {
        render_directory(basedir, s, dir)?;
    }
    for md in s.iter_child_modules(d) {
        render_module_config(basedir, s, md)?;
    }

    render_config_index(
        &dir_path,
        s.iter_child_directories(d),
        s.iter_child_modules(d),
    )?;

    Ok(())
}

fn render_config_index<'a>(
    dir_path: &PathBuf,
    dirs: impl Iterator<Item = &'a Directory>,
    mds: impl Iterator<Item = &'a Module>,
) -> Result<()> {
    let filepath = dir_path.join("__init__.py");
    let mut file = File::create(filepath)?;

    let mut src = SourceCode::new();

    src.import_third("typing");

    src.line("class Config(typing.NamedTuple):");
    src.indent();
    for dir in dirs {
        let name = dir.path.file_name().unwrap().to_str().unwrap();
        src.import_first(&format!(".{name}"));
        src.line(&format!(
            "{name}: {type}.Config",
            name = name,
            type = name,
        ));
    }
    for md in mds {
        let name = md.path.file_stem().unwrap().to_str().unwrap();
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

fn render_module_config(basedir: &PathBuf, s: &Schema, m: &Module) -> Result<()> {
    let filepath = basedir.join(m.path.with_extension("py"));
    let mut file = File::create(filepath)?;

    let mut src = SourceCode::new();

    src.import_third("typing");

    src.line("class Config(typing.NamedTuple):");
    src.indent();

    let mut pass = true;

    for def in &m.definitions {
        pass = false;
        match def {
            Definition::TypeDefinition(inner) => match inner {
                TypeDefinition::Object(inner) => {
                    render_object_def_resolver(&mut src, s, m, inner);
                    src.line("");
                }
                TypeDefinition::Interface(inner) => {
                    render_interface_def_resolver(&mut src, s, m, inner);
                    src.line("");
                }
                TypeDefinition::Scalar(inner) => {
                    // render_scalar_def_resolver(src, s, inner);
                }
                TypeDefinition::Input(inner) => { /* noop */ }
                TypeDefinition::Enum(inner) => { /* noop */ }
                TypeDefinition::Union(inner) => { /* noop */ }
            },
            Definition::TypeExtension(inner) => match inner {
                TypeExtension::ObjectExtension(inner) => {
                    render_object_ext_resolver(&mut src, s, m, inner);
                    src.line("");
                }
                TypeExtension::InterfaceExtension(inner) => {
                    render_interface_ext_resolver(&mut src, s, m, inner);
                    src.line("");
                }
                TypeExtension::InputExtension(inner) => { /* noop */ }
                TypeExtension::EnumExtension(inner) => { /* noop */ }
                TypeExtension::UnionExtension(inner) => { /* noop */ }
            },
            _ => {}
        }
    }

    for def in &m.definitions {
        match def {
            Definition::TypeDefinition(inner) => match inner {
                TypeDefinition::Object(inner) => {
                    src.line(&format!(
                        "{name}: {type}",
                        name = inner.name,
                        type = naming::def_object_resolver_type(&inner),
                    ));
                }
                TypeDefinition::Interface(inner) => {
                    src.line(&format!(
                        "{name}: {type}",
                        name = inner.name,
                        type = naming::def_interface_resolver_type(&inner),
                    ));
                }
                TypeDefinition::Scalar(inner) => {
                    // render_scalar_def_resolver(src, s, inner);
                }
                TypeDefinition::Input(inner) => { /* noop */ }
                TypeDefinition::Enum(inner) => { /* noop */ }
                TypeDefinition::Union(inner) => { /* noop */ }
            },
            Definition::TypeExtension(inner) => match inner {
                TypeExtension::ObjectExtension(inner) => {
                    src.line(&format!(
                        "{name}: {type}",
                        name = inner.name,
                        type = naming::ext_object_resolver_type(&inner),
                    ));
                }
                TypeExtension::InterfaceExtension(inner) => {
                    src.line(&format!(
                        "{name}: {type}",
                        name = inner.name,
                        type = naming::ext_interface_resolver_type(&inner),
                    ));
                }
                TypeExtension::InputExtension(inner) => { /* noop */ }
                TypeExtension::EnumExtension(inner) => { /* noop */ }
                TypeExtension::UnionExtension(inner) => { /* noop */ }
            },
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

fn render_object_def_resolver(src: &mut SourceCode, s: &Schema, m: &Module, def: &Object) {
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

fn render_interface_def_resolver(src: &mut SourceCode, s: &Schema, m: &Module, def: &Interface) {
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
    def: &InterfaceTypeExtension,
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
