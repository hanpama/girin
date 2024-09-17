use super::{error::Result, naming, source_code::SourceCode};
use crate::schema::{Definition, Field, ModuleRef, ObjectDefinition, ObjectExtension, Project};
use std::{fs::File, path::PathBuf};

pub fn render(outdir: &PathBuf, s: &Project) -> Result<()> {
    let root = outdir.join("RuntimeWiring");

    let d = ModuleRef::new(s);

    std::fs::create_dir_all(&root)?;

    for child in d.iter_children() {
        render_directory(&root, child)?;
    }
    render_root_config_index(&root, &d)?;

    Ok(())
}

fn render_directory(parent_path: &PathBuf, d: ModuleRef) -> Result<()> {
    if d.has_children() {
        let module_name = naming::module_name(&d.get_name());
        let path = parent_path.join(&module_name);
        std::fs::create_dir_all(&path)?;
        for child in d.iter_children() {
            render_directory(&path, child)?;
        }
        render_config_index(&path, &d)?;
    } else {
        render_config_index(&parent_path, &d)?;
    }
    Ok(())
}

fn render_root_config_index(outdir: &PathBuf, d: &ModuleRef) -> Result<()> {
    let filepath = outdir.join("RuntimeWiring.swift");
    let mut file = File::create(filepath)?;
    let mut src = SourceCode::new();

    src.line("extension Runtime.Wiring {");
    src.indent();

    src.line("init() {");
    src.indent();
    if d.has_children() {
        for dir in d.iter_children() {
            let name = naming::module_name(dir.get_name());
            src.line(&format!("self.{name} = .init()",));
        }
    }
    src.dedent();
    src.line("}");

    src.dedent();
    src.line("}");

    src.write_to(&mut file)?;

    Ok(())
}

fn render_config_index<'a>(outdir: &PathBuf, d: &ModuleRef) -> Result<()> {
    let breadcrumbs = d.get_breadcrumbs();
    let filepath = outdir.join(format!("RuntimeWiring${}.swift", breadcrumbs.join("$")));
    let mut file = File::create(filepath)?;
    let mut src = SourceCode::new();

    let extention_target = format!("Runtime.Wiring.{}", naming::module_path(&breadcrumbs));

    src.line(format!("extension {extention_target} {{"));
    src.indent();

    let has_init = d.has_children()
        || d.iter_definitions().any(|def| match def {
            Definition::ObjectDefinition(_) => true,
            Definition::ObjectExtension(_) => true,
            _ => false,
        });

    if has_init {
        src.line("init() {");
        src.indent();

        if d.has_children() {
            for dir in d.iter_children() {
                let name = naming::module_name(dir.get_name());
                src.line(&format!("self.{name} = .init()",));
            }
        }

        if d.has_definition() {
            for def in d.iter_definitions() {
                match def {
                    Definition::ObjectDefinition(inner) => {
                        render_object_config(&d, &mut src, d.schema, inner);
                    }
                    Definition::ObjectExtension(inner) => {
                        render_object_ext_config(&d, &mut src, d.schema, inner);
                    }
                    _ => { /* noop */ }
                }
            }
        }
        src.dedent();
        src.line("}");
    }

    src.dedent();
    src.line("}");

    src.write_to(&mut file)?;

    Ok(())
}

fn render_object_config(d: &ModuleRef, src: &mut SourceCode, s: &Project, def: &ObjectDefinition) {
    let impl_name = naming::runtime_spec(&def.name);
    src.line(format!("self.{impl_name} = .init("));
    src.indent();

    for (i, field) in def
        .iter_fields()
        .filter(|field| field.has_resolve_config())
        .enumerate()
    {
        if i > 0 {
            src.append(",");
        }
        render_field_resolver(d, src, &field);
    }

    src.dedent();
    src.line(")");
}

fn render_object_ext_config(
    d: &ModuleRef,
    src: &mut SourceCode,
    s: &Project,
    def: &ObjectExtension,
) {
    let impl_name = naming::runtime_spec(&def.name);
    src.line(format!("self.{impl_name} = .init("));
    src.indent();

    for (i, field) in def
        .iter_fields()
        .filter(|field| field.has_resolve_config())
        .enumerate()
    {
        if i > 0 {
            src.append(",");
        }
        render_field_resolver(d, src, &field);
    }

    src.dedent();
    src.line(")");
}

fn render_field_resolver(d: &ModuleRef, src: &mut SourceCode, field: &Field) {
    let name = naming::field_name(&field.name);
    src.line(&format!("{name}: {{ source, args, context, info in",));

    src.indent();
    src.line("fatalError(\"Not implemented\")");
    src.dedent();
    src.line("}");
}
