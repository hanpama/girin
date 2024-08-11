use super::{error::PythonRenderingError, naming, sourcecode::SourceCode};
use crate::schema::{
    Directory, Interface, InterfaceTypeExtension, Object, ObjectExtension, Schema, SchemaElement,
};
use std::{fs::File, path::PathBuf};

pub fn render_builder_config(outdir: &PathBuf, s: &Schema) -> Result<(), PythonRenderingError> {
    let root_dir = outdir.join("builder_config");

    // s.iter_modules(path)

    // if pass {
    //     src.line("pass");
    // }
    // src.dedent();

    for dir in s.iter_root_directories() {
        render_directory(&root_dir, s, dir)?;
    }

    Ok(())
}

fn render_directory(
    basedir: &PathBuf,
    s: &Schema,
    d: &Directory,
) -> Result<(), PythonRenderingError> {
    // mkdir -p
    let dir_path = basedir.join(&d.path);
    std::fs::create_dir_all(&dir_path)?;

    // s.iter_child_directories(path)


    Ok(())
}

// fn render_file(basedir: &PathBuf, s: &Schema) {
//     render_module_config(src, s, md);
//     // md.
// }

// fn render_module_config(src: &mut SourceCode, s: &Schema, md: &Module) {
//     src.import_third("typing");

//     src.line("class Module(typing.NamedTuple):");
//     src.indent();

//     let mut pass = true;
//     // for child in &md.children {
//     //     src.line(&format!(
//     //         "{name}: {type}",
//     //         name = naming::module_config_field_name(child),
//     //         type = naming::module_config_type(child)
//     //     ));
//     // }
//     // for smd in &md.submodules {
//     //     src.line(&format!(
//     //         "{name}: {type}",
//     //         name = naming::submodule_config_field_name(smd),
//     //         type = naming::submodule_config_type(smd)
//     //     ));
//     // }

//     if pass {
//         src.line("pass");
//     }

//     src.dedent();
// }

// fn render_submodule_config(src: &mut SourceCode, s: &Schema, smd: &Submodule) {
//     src.import_third("typing");

//     src.line(&format!(
//         "class {name}(typing.NamedTuple):",
//         name = naming::submodule_config_type(&smd)
//     ));
//     src.indent();

//     let mut pass = true;
//     for def in &smd.definitions {
//         pass = false;
//         match def {
//             SchemaElement::Object(inner) => {
//                 render_object_def_resolver(src, s, inner);
//             }
//             SchemaElement::Interface(inner) => {
//                 render_interface_def_resolver(src, s, inner);
//             }
//             SchemaElement::ObjectExtension(inner) => {
//                 render_object_ext_resolver(src, s, inner);
//             }
//             SchemaElement::InterfaceExtension(inner) => {
//                 render_interface_ext_resolver(src, s, inner);
//             }
//             _ => {}
//         }
//     }

//     // extension resolvers
//     // scalar config

//     // for def in &smd.definitions {
//     //     src.line(&format!(
//     //         "{name}: {type}",
//     //         name = def.name,
//     //         type = format_module_type_name(def)
//     //     ));
//     // }

//     // for child in &md.children {
//     //     src.line(&format!(
//     //         "{name}: {type}",
//     //         name = child.name,
//     //         type = format_module_type_name(child)
//     //     ));
//     // }

//     if pass {
//         src.line("pass");
//     }

//     src.dedent();
// }

// fn render_object_def_resolver(src: &mut SourceCode, s: &Schema, def: &Object) {
//     src.line(&format!(
//         "class {name}(typing.Protocol):",
//         name = naming::object_def_resolver_type(def)
//     ));
//     src.indent();

//     for field in &def.fields {
//         src.import_first(".source");
//         src.line(&format!(
//             "def {field}(self, obj: \"source.{obj_type}\", info, **args): ...",
//             field = field.name,
//             obj_type = naming::object_source(def),
//         ));
//     }
//     src.dedent();
// }

// fn render_interface_def_resolver(src: &mut SourceCode, s: &Schema, def: &Interface) {
//     src.line(&format!(
//         "class {name}(typing.Protocol):",
//         name = naming::interface_def_resolver_type(def)
//     ));
//     src.indent();
//     for field in &def.fields {
//         src.import_first(".source");
//         src.line(&format!(
//             "def {field}(self, obj: \"source.{obj_type}\", info, **args): ...",
//             field = field.name,
//             obj_type = naming::interface_source(def),
//         ));
//     }
//     src.dedent();
// }

// fn render_object_ext_resolver(src: &mut SourceCode, s: &Schema, def: &ObjectExtension) {
//     src.line(&format!(
//         "class {name}(typing.Protocol):",
//         name = naming::object_ext_resolver_type(def)
//     ));
//     src.indent();

//     for field in &def.fields {
//         src.import_first(".source");
//         src.line(&format!(
//             "def {field}(self, obj: \"source.{obj_type}\", info, **args): ...",
//             field = field.name,
//             obj_type = naming::source(&def.name),
//         ));
//     }
//     src.dedent();
// }

// fn render_interface_ext_resolver(src: &mut SourceCode, s: &Schema, def: &InterfaceTypeExtension) {
//     src.line(&format!(
//         "class {name}(typing.Protocol):",
//         name = naming::interface_ext_resolver_type(def)
//     ));
//     src.indent();
//     for field in &def.fields {
//         src.import_first(".source");
//         src.line(&format!(
//             "def {field}(self, obj: \"source.{obj_type}\", info, **args): ...",
//             field = field.name,
//             obj_type = naming::source(&def.name),
//         ));
//     }
//     src.dedent();
// }
