use super::{error::PythonRenderingError, naming, sourcecode::SourceCode};
use crate::definitions::{
    Definition, Interface, InterfaceTypeExtension, Module, Object,
    ObjectExtension, Schema, Submodule,
};
use std::{fs::File, path::PathBuf};

pub fn render_builder_config(outdir: &PathBuf, s: &Schema) -> Result<(), PythonRenderingError> {
    let outfile = outdir.join("builder_config.py");

    let mut file = File::create(outfile)?;
    let mut src = SourceCode::new();

    src.line("class BuilderConfig:");
    src.indent();

    let mut pass = true;
    for child in &s.children {
        pass = false;
        render_module_config(&mut src, s, child);
    }
    for smd in &s.submodules {
        pass = false;
        render_submodule_config(&mut src, s, smd);
    }

    for child in &s.children {
        src.line(&format!(
            "{name}: {type}",
            name = naming::module_config_field_name(child),
            type = naming::module_config_type(child)
        ));
    }
    for smd in &s.submodules {
        src.line(&format!(
            "{name}: {type}",
            name = naming::submodule_config_field_name(smd),
            type = naming::submodule_config_type(smd)
        ));
    }

    if pass {
        src.line("pass");
    }
    src.dedent();

    src.write_to(&mut file)?;

    Ok(())
}

fn render_module(src: &mut SourceCode, s: &Schema, md: &Module) {
    render_module_config(src, s, md);
    // md.
}

fn render_module_config(src: &mut SourceCode, s: &Schema, md: &Module) {
    src.import_third("typing");

    src.line(&format!(
        "class {name}(typing.NamedTuple):",
        name = naming::module_config_type(md)
    ));
    src.indent();

    let mut pass = true;
    for child in &md.children {
        pass = false;
        render_module_config(src, s, child);
    }
    for smd in &md.submodules {
        pass = false;
        render_submodule_config(src, s, smd);
    }

    for child in &md.children {
        src.line(&format!(
            "{name}: {type}",
            name = naming::module_config_field_name(child),
            type = naming::module_config_type(child)
        ));
    }
    for smd in &md.submodules {
        src.line(&format!(
            "{name}: {type}",
            name = naming::submodule_config_field_name(smd),
            type = naming::submodule_config_type(smd)
        ));
    }

    if pass {
        src.line("pass");
    }

    src.dedent();
}

fn render_submodule_config(src: &mut SourceCode, s: &Schema, smd: &Submodule) {
    src.import_third("typing");

    src.line(&format!(
        "class {name}(typing.NamedTuple):",
        name = naming::submodule_config_type(&smd)
    ));
    src.indent();

    let mut pass = true;
    for def in &smd.definitions {
        pass = false;
        match def {
            Definition::Object(inner) => {
                render_object_def_resolver(src, s, inner);
            }
            Definition::Interface(inner) => {
                render_interface_def_resolver(src, s, inner);
            }
            Definition::ObjectExtension(inner) => {
                render_object_ext_resolver(src, s, inner);
            }
            Definition::InterfaceExtension(inner) => {
                render_interface_ext_resolver(src, s, inner);
            }
            _ => {}
        }
    }

    // extension resolvers
    // scalar config

    // for def in &smd.definitions {
    //     src.line(&format!(
    //         "{name}: {type}",
    //         name = def.name,
    //         type = format_module_type_name(def)
    //     ));
    // }

    // for child in &md.children {
    //     src.line(&format!(
    //         "{name}: {type}",
    //         name = child.name,
    //         type = format_module_type_name(child)
    //     ));
    // }

    if pass {
        src.line("pass");
    }

    src.dedent();
}

fn render_object_def_resolver(src: &mut SourceCode, s: &Schema, def: &Object) {
    src.line(&format!(
        "class {name}(typing.Protocol):",
        name = naming::object_def_resolver_type(def)
    ));
    src.indent();

    for field in &def.fields {
        src.import_first(".source");
        src.line(&format!(
            "def {field}(self, obj: \"source.{obj_type}\", info, **args): ...",
            field = field.name,
            obj_type = naming::object_source(def),
        ));
    }
    src.dedent();
}

fn render_interface_def_resolver(src: &mut SourceCode, s: &Schema, def: &Interface) {
    src.line(&format!(
        "class {name}(typing.Protocol):",
        name = naming::interface_def_resolver_type(def)
    ));
    src.indent();
    for field in &def.fields {
        src.import_first(".source");
        src.line(&format!(
            "def {field}(self, obj: \"source.{obj_type}\", info, **args): ...",
            field = field.name,
            obj_type = naming::interface_source(def),
        ));
    }
    src.dedent();
}

fn render_object_ext_resolver(src: &mut SourceCode, s: &Schema, def: &ObjectExtension) {
    src.line(&format!(
        "class {name}(typing.Protocol):",
        name = naming::object_ext_resolver_type(def)
    ));
    src.indent();

    for field in &def.fields {
        src.import_first(".source");
        src.line(&format!(
            "def {field}(self, obj: \"source.{obj_type}\", info, **args): ...",
            field = field.name,
            obj_type = naming::source(&def.name),
        ));
    }
    src.dedent();
}

fn render_interface_ext_resolver(src: &mut SourceCode, s: &Schema, def: &InterfaceTypeExtension) {
    src.line(&format!(
        "class {name}(typing.Protocol):",
        name = naming::interface_ext_resolver_type(def)
    ));
    src.indent();
    for field in &def.fields {
        src.import_first(".source");
        src.line(&format!(
            "def {field}(self, obj: \"source.{obj_type}\", info, **args): ...",
            field = field.name,
            obj_type = naming::source(&def.name),
        ));
    }
    src.dedent();
}
