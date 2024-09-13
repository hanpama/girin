use super::{error::Result, naming, source_code::SourceCode, type_expr::format_type_expression};
use crate::schema::{
    Definition, InputValue, InterfaceDefinition, InterfaceExtension, ModuleRef, ObjectDefinition,
    ObjectExtension, Project, Resolve, ScalarDefinition,
};
use std::{fs::File, path::PathBuf};

pub fn render(outdir: &PathBuf, s: &Project) -> Result<()> {
    let root = outdir.join("Runtime");
    std::fs::create_dir_all(&root)?;

    let d = ModuleRef::new(s);

    std::fs::create_dir_all(&root)?;
    for child in d.iter_children() {
        render_directory(&root, child)?;
    }
    render_config_index(&root, &d)?;

    Ok(())
}

fn render_directory(parent_path: &PathBuf, d: ModuleRef) -> Result<()> {
    let module_name = naming::module_name(&d.get_name());
    let path = parent_path.join(&module_name);

    std::fs::create_dir_all(&path)?;
    if d.has_children() {
        for child in d.iter_children() {
            render_directory(&path.join(child.get_name()), child)?;
        }
        render_config_index(&path, &d)?;
    } else {
        // has_definition
        render_module_config(&path, &d)?;
    }

    Ok(())
}

fn render_config_index<'a>(outdir: &PathBuf, d: &ModuleRef) -> Result<()> {
    let breadcrumbs = d.get_breadcrumbs();
    let filename = breadcrumbs.join("$");
    let filepath = outdir.join(filename).with_extension("swift");

    println!("render_config_index: {:?}", filepath);
    let mut file = File::create(filepath)?;

    let mut src = SourceCode::new();

    let breadcrumbs = d.get_breadcrumbs();

    let name = if breadcrumbs.is_empty() {
        "RuntimeSpec"
    } else {
        let module_accessor = naming::module_path(&breadcrumbs);
        &format!("RuntimeSpec.{}", module_accessor)
    };

    src.line(format!("extension {name} {{"));
    src.indent();

    src.line(".init() {");
    src.indent();
    for dir in d.iter_children() {
        let name = naming::module_name(dir.get_name());
        src.line(&format!("self.{name} = .init()",));
    }
    src.dedent();
    src.line("}");

    src.dedent();
    src.line("}");

    src.write_to(&mut file)?;

    Ok(())
}

fn render_module_config(outdir: &PathBuf, d: &ModuleRef) -> Result<()> {
    let breadcrumbs = d.get_breadcrumbs();
    let filename = breadcrumbs.join("$");
    let filepath = outdir.join(filename).with_extension("swift");

    if filepath.exists() {
        return Ok(());
    }

    println!("render_module_config: {:?}", filepath);
    let mut file = File::create(filepath)?;

    let mut src = SourceCode::new();

    // src.import(&format!(
    //     "from {}. import runtime_spec",
    //     ".".repeat(d.get_depth())
    // ));
    let module_accessor = naming::module_path(&d.get_breadcrumbs());

    src.line(format!("extension RuntimeSpec.{module_accessor} {{"));
    src.indent();

    src.line("init() {");
    src.indent();

    for def in d.iter_definitions() {
        match def {
            Definition::ObjectDefinition(inner) => {
                render_object_config(&d, &mut src, d.schema, inner);
            }
            Definition::InterfaceDefinition(inner) => {
                render_interface_config(&d, &mut src, d.schema, inner);
            }
            Definition::ScalarDefinition(inner) => {
                render_scalar_config(&d, &mut src, d.schema, inner);
            }

            // Definition::ObjectExtension(inner) => {
            //     render_object_ext_config(&d, &mut src, d.schema, inner);
            //     src.newline();
            //     src.newline();
            // }
            // Definition::InterfaceExtension(inner) => {
            //     render_interface_ext_config(&d, &mut src, d.schema, inner);
            //     src.newline();
            //     src.newline();
            // }
            _ => { /* noop */ }
        }
    }

    src.dedent();
    src.line("}");

    src.dedent();
    src.line("}");

    src.write_to(&mut file)?;

    Ok(())
}

fn render_object_config(d: &ModuleRef, src: &mut SourceCode, s: &Project, def: &ObjectDefinition) {
    let impl_name = naming::runtime_spec(&def.name);
    src.line(format!("self.{impl_name} = .init("));
    src.indent();

    let resolves = def
        .iter_fields()
        .filter_map(|field| s.resolve_field_resolve(field));

    for (i, resolve) in resolves.enumerate() {
        if i > 0 {
            src.append(",");
        }
        render_field_resolver(d, src, &resolve);
    }

    src.dedent();
    src.line(")");
}

fn render_interface_config(
    d: &ModuleRef,
    src: &mut SourceCode,
    s: &Project,
    def: &InterfaceDefinition,
) {
    let impl_name = naming::runtime_spec(&def.name);
    src.line(format!("self.{impl_name} = .init("));
    src.indent();

    let resolves = def
        .iter_fields()
        .filter_map(|field| s.resolve_field_resolve(field));

    for (i, resolve) in resolves.enumerate() {
        if i > 0 {
            src.append(",");
        }
        render_field_resolver(d, src, &resolve);
    }

    src.dedent();
    src.line(")");
}

fn render_scalar_config(d: &ModuleRef, src: &mut SourceCode, s: &Project, def: &ScalarDefinition) {
    let impl_name = naming::runtime_spec(&def.name);
    src.line(format!("self.{impl_name} = .init("));
    src.indent();
    src.line("serialize: { val in");
    src.indent();
    src.line("fatalError(\"Not implemented\")");
    src.dedent();
    src.line("},");
    src.line("parseValue: { val in");
    src.indent();
    src.line("fatalError(\"Not implemented\")");
    src.dedent();
    src.line("},");
    src.line("parseLiteral: { val in");
    src.indent();
    src.line("fatalError(\"Not implemented\")");
    src.dedent();
    src.line("}");
    src.dedent();
    src.line(")");
}

// fn render_object_ext_config(
//     d: &ModuleRef,
//     src: &mut SourceCode,
//     s: &Project,
//     def: &ObjectExtension,
// ) {
//     let module_accessor = naming::module_path(&d.get_breadcrumbs());
//     let impl_name = naming::impl_type(&def.name);
//     let spec_name = naming::runtime_spec_type(&def.name);

//     src.import("import typing");
//     src.line("@typing.final");
//     src.line(&format!(
//         "class {impl_name}(runtime_spec.{module_accessor}.{spec_name}):",
//     ));
//     src.indent();

//     let mut pass = true;

//     for field in def.iter_fields() {
//         if let Some(resolve) = s.resolve_field_resolve(field) {
//             pass = false;
//             render_field_resolver(d, src, &resolve);
//             src.newline();
//         }
//     }

//     if pass {
//         src.line("pass");
//     }
//     src.dedent();
// }

// fn render_interface_ext_config(
//     d: &ModuleRef,
//     src: &mut SourceCode,
//     s: &Project,
//     def: &InterfaceExtension,
// ) {
//     let module_accessor = naming::module_path(&d.get_breadcrumbs());
//     let impl_name = naming::impl_type(&def.name);
//     let spec_name = naming::runtime_spec_type(&def.name);

//     src.import("import typing");
//     src.line("@typing.final");
//     src.line(&format!(
//         "class {impl_name}(runtime_spec.{module_accessor}.{spec_name}):",
//     ));
//     src.indent();
//     let mut pass = true;

//     for field in def.iter_fields() {
//         if let Some(resolve) = s.resolve_field_resolve(field) {
//             pass = false;
//             render_field_resolver(d, src, &resolve);
//             src.newline();
//         }
//     }

//     if pass {
//         src.line("pass");
//     }
//     src.dedent();
// }

fn render_field_resolver(d: &ModuleRef, src: &mut SourceCode, resolve: &Resolve) {
    let name = naming::field_name(&resolve.field.name);
    let sig = if resolve.sync {
        "throws"
    } else {
        "async throws"
    };

    src.line(&format!("{name}: {{ src, info, args {sig} in",));

    src.indent();
    src.line("fatalError(\"Not implemented\")");
    src.dedent();
    src.line("}");
}

fn format_argument_list(args: &Vec<InputValue>) -> String {
    args.iter()
        .map(|input| {
            let name = naming::field_name(&input.name);
            let expr = format_type_expression(Some("source_spec"), &input.field_type);
            if input.field_type.is_nullable() {
                format!("{name}: {expr} = None")
            } else {
                format!("{name}: {expr}")
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}
