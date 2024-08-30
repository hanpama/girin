use super::{
    error::Result, format_named_source, format_type_expression, naming, sourcecode::SourceCode,
};
use crate::schema::{
    Definition, InterfaceDefinition, InterfaceExtension, ObjectDefinition, ObjectExtension,
    Resolve, ScalarDefinition, Schema, ModuleRef,
};
use std::{fs::File, path::PathBuf};

pub fn render(outdir: &PathBuf, s: &Schema) -> Result<()> {
    render_directory(outdir.join("impl"), ModuleRef::new(s))?;
    Ok(())
}

fn render_directory(path: PathBuf, d: ModuleRef) -> Result<()> {
    if d.has_children() {
        std::fs::create_dir_all(&path)?;
        for child in d.iter_children() {
            let child_path = path.join(child.get_name());
            render_directory(child_path, child)?;
        }
        render_config_index(&path, &d)?;
    }
    if d.has_definition() {
        render_module_config(&path.with_extension("py"), &d)?;
    }

    Ok(())
}

fn render_config_index<'a>(parent_path: &PathBuf, d: &ModuleRef) -> Result<()> {
    let filepath = parent_path.join("__init__.py");
    let mut file = File::create(filepath)?;

    let mut src = SourceCode::new_generated();

    src.line("__all__ = [");
    src.indent();
    for dir in d.iter_children() {
        let name = dir.get_name();
        src.import(&format!("from . import {name}"));
        src.line(&format!("\"{name}\",",));
    }
    src.dedent();
    src.line("]");

    src.write_to(&mut file)?;

    Ok(())
}

fn render_module_config(filepath: &PathBuf, d: &ModuleRef) -> Result<()> {
    if filepath.exists() {
        return Ok(());
    }

    let mut file = File::create(filepath)?;

    let mut src = SourceCode::new();

    src.import(&format!(
        "from {}.spec import Spec",
        ".".repeat(d.get_depth())
    ));

    for def in d.iter_definitions() {
        match def {
            Definition::ObjectDefinition(inner) => {
                render_object_config(&d, &mut src, d.schema, inner);
                src.line("");
                src.line("");
            }
            Definition::InterfaceDefinition(inner) => {
                render_interface_config(&d, &mut src, d.schema, inner);
                src.line("");
                src.line("");
            }
            Definition::ScalarDefinition(inner) => {
                render_scalar_config(&d, &mut src, d.schema, inner);
                src.line("");
                src.line("");
            }

            Definition::ObjectExtension(inner) => {
                render_object_ext_config(&d, &mut src, d.schema, inner);
                src.line("");
                src.line("");
            }
            Definition::InterfaceExtension(inner) => {
                render_interface_ext_config(&d, &mut src, d.schema, inner);
                src.line("");
                src.line("");
            }
            _ => { /* noop */ }
        }
    }

    src.write_to(&mut file)?;

    Ok(())
}

fn render_object_config(d: &ModuleRef, src: &mut SourceCode, s: &Schema, def: &ObjectDefinition) {
    let module_accessor = d.get_breadcrumbs().join(".");
    let impl_name = naming::impl_type(&def.name);
    let spec_name = naming::spec_type(&def.name);

    src.line(&format!(
        "class {impl_name}(Spec.{module_accessor}.{spec_name}):",
    ));
    src.indent();

    let mut pass = true;

    for field in def.iter_fields() {
        if let Some(resolve) = s.resolve_field_resolve(field) {
            pass = false;
            render_field_resolver(d, src, &resolve);
            src.line("");
        }
    }

    if pass {
        src.line("pass");
    }
    src.dedent();
}

fn render_interface_config(
    d: &ModuleRef,
    src: &mut SourceCode,
    s: &Schema,
    def: &InterfaceDefinition,
) {
    // def.module
    let module_accessor = d.get_breadcrumbs().join(".");
    let impl_name = naming::impl_type(&def.name);
    let spec_name = naming::spec_type(&def.name);

    src.line(&format!(
        "class {impl_name}(Spec.{module_accessor}.{spec_name}):",
    ));
    src.indent();

    let mut pass = true;

    for field in def.iter_fields() {
        if let Some(resolve) = s.resolve_field_resolve(field) {
            pass = false;
            render_field_resolver(d, src, &resolve);
            src.line("");
        }
    }

    if pass {
        src.line("pass");
    }
    src.dedent();
}

fn render_scalar_config(d: &ModuleRef, src: &mut SourceCode, s: &Schema, def: &ScalarDefinition) {
    let module_accessor = d.get_breadcrumbs().join(".");
    let impl_name = naming::impl_type(&def.name);
    let spec_name = naming::spec_type(&def.name);

    src.import("import typing");
    src.import("import graphql");
    src.line(&format!(
        "class {impl_name}(Spec.{module_accessor}.{spec_name}):"
    ));
    src.indent();

    src.line("def serialize(self, value: typing.Any) -> typing.Any:");
    src.indent();
    src.line("raise NotImplementedError()");
    src.dedent();
    src.line("");
    src.line("def parse_value(self, value: typing.Any) -> typing.Any:");
    src.indent();
    src.line("raise NotImplementedError()");
    src.dedent();
    src.line("");
    src.line("def parse_literal(self, node: graphql.ValueNode, variables) -> typing.Any:");
    src.indent();
    src.line("raise NotImplementedError()");
    src.dedent();

    src.dedent();
}

fn render_object_ext_config(
    d: &ModuleRef,
    src: &mut SourceCode,
    s: &Schema,
    def: &ObjectExtension,
) {
    let module_accessor = d.get_breadcrumbs().join(".");
    let impl_name = naming::impl_type(&def.name);
    let spec_name = naming::spec_type(&def.name);

    src.line(&format!(
        "class {impl_name}(Spec.{module_accessor}.{spec_name}):",
    ));
    src.indent();

    let mut pass = true;

    for field in def.iter_fields() {
        if let Some(resolve) = s.resolve_field_resolve(field) {
            pass = false;
            render_field_resolver(d, src, &resolve);
            src.line("");
        }
    }

    if pass {
        src.line("pass");
    }
    src.dedent();
}

fn render_interface_ext_config(
    d: &ModuleRef,
    src: &mut SourceCode,
    s: &Schema,
    def: &InterfaceExtension,
) {
    let module_accessor = d.get_breadcrumbs().join(".");
    let impl_name = naming::impl_type(&def.name);
    let spec_name = naming::spec_type(&def.name);

    src.line(&format!(
        "class {impl_name}(Spec.{module_accessor}.{spec_name}):",
    ));
    src.indent();
    let mut pass = true;

    for field in def.iter_fields() {
        if let Some(resolve) = s.resolve_field_resolve(field) {
            pass = false;
            render_field_resolver(d, src, &resolve);
            src.line("");
        }
    }

    if pass {
        src.line("pass");
    }
    src.dedent();
}

fn render_field_resolver(d: &ModuleRef, src: &mut SourceCode, resolve: &Resolve) {
    src.import(&format!(
        "from {} import source",
        ".".repeat(d.get_depth() + 1)
    ));

    let name = naming::field_name(&resolve.field.name);
    let source_type = format_named_source(&resolve.field.type_name);
    let return_type = format_type_expression(&resolve.field.field_type);
    let sig = if resolve.sync { "def" } else { "async def" };

    src.line(&format!(
        "{sig} {name}(self, obj: {source_type}, info, **args) -> {return_type}:",
    ));

    src.indent();
    src.line("raise NotImplementedError()");
    src.dedent();
}
