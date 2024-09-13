use super::{error::Result, format_type_expression, naming, source_code::SourceCode};
use crate::schema::{
    Definition, InputValue, InterfaceDefinition, InterfaceExtension, ModuleRef, ObjectDefinition,
    ObjectExtension, Project, Resolve, ScalarDefinition,
};
use std::{fs::File, path::PathBuf};

pub fn render(outdir: &PathBuf, s: &Project) -> Result<()> {
    let root = outdir.join("runtime");
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

    if d.has_children() {
        println!("Creating directory: {:?}", &path);
        std::fs::create_dir_all(&path)?;

        for child in d.iter_children() {
            render_directory(&path, child)?;
        }
        render_config_index(&path, &d)?;
    } else {
        // has_definition
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
        let name = naming::module_name(dir.get_name());
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
        "from {}. import runtime_spec",
        ".".repeat(d.get_depth())
    ));

    for def in d.iter_definitions() {
        match def {
            Definition::ObjectDefinition(inner) => {
                render_object_config(&d, &mut src, d.schema, inner);
                src.newline();
                src.newline();
            }
            Definition::InterfaceDefinition(inner) => {
                render_interface_config(&d, &mut src, d.schema, inner);
                src.newline();
                src.newline();
            }
            Definition::ScalarDefinition(inner) => {
                render_scalar_config(&d, &mut src, d.schema, inner);
                src.newline();
                src.newline();
            }

            Definition::ObjectExtension(inner) => {
                render_object_ext_config(&d, &mut src, d.schema, inner);
                src.newline();
                src.newline();
            }
            Definition::InterfaceExtension(inner) => {
                render_interface_ext_config(&d, &mut src, d.schema, inner);
                src.newline();
                src.newline();
            }
            _ => { /* noop */ }
        }
    }

    src.write_to(&mut file)?;

    Ok(())
}

fn render_object_config(d: &ModuleRef, src: &mut SourceCode, s: &Project, def: &ObjectDefinition) {
    let module_accessor = naming::module_path(&d.get_breadcrumbs());
    let impl_name = naming::impl_type(&def.name);
    let spec_name = naming::runtime_spec_type(&def.name);

    src.import("import typing");
    src.line("@typing.final");
    src.line(&format!(
        "class {impl_name}(runtime_spec.{module_accessor}.{spec_name}):",
    ));
    src.indent();

    let mut pass = true;

    for field in def.iter_fields() {
        if let Some(resolve) = s.resolve_field_resolve(field) {
            pass = false;
            render_field_resolver(d, src, &resolve);
            src.newline();
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
    s: &Project,
    def: &InterfaceDefinition,
) {
    let module_accessor = naming::module_path(&d.get_breadcrumbs());
    let impl_name = naming::impl_type(&def.name);
    let spec_name = naming::runtime_spec_type(&def.name);

    src.import("import typing");
    src.line("@typing.final");
    src.line(&format!(
        "class {impl_name}(runtime_spec.{module_accessor}.{spec_name}):",
    ));
    src.indent();

    let mut pass = true;

    for field in def.iter_fields() {
        if let Some(resolve) = s.resolve_field_resolve(field) {
            pass = false;
            render_field_resolver(d, src, &resolve);
            src.newline();
        }
    }

    if pass {
        src.line("pass");
    }
    src.dedent();
}

fn render_scalar_config(d: &ModuleRef, src: &mut SourceCode, s: &Project, def: &ScalarDefinition) {
    let module_accessor = naming::module_path(&d.get_breadcrumbs());
    let impl_name = naming::impl_type(&def.name);
    let spec_name = naming::runtime_spec_type(&def.name);

    src.import("import typing");
    src.import("import graphql");

    src.line("@typing.final");
    src.line(&format!(
        "class {impl_name}(runtime_spec.{module_accessor}.{spec_name}):"
    ));
    src.indent();

    src.line("def serialize(self, value: typing.Any) -> typing.Any:");
    src.indent();
    src.line("raise NotImplementedError()");
    src.dedent();
    src.newline();
    src.line("def parse_value(self, value: typing.Any) -> typing.Any:");
    src.indent();
    src.line("raise NotImplementedError()");
    src.dedent();
    src.newline();
    src.line(
        "def parse_literal(self, node: graphql.ValueNode, variables: typing.Any) -> typing.Any:",
    );
    src.indent();
    src.line("raise NotImplementedError()");
    src.dedent();

    src.dedent();
}

fn render_object_ext_config(
    d: &ModuleRef,
    src: &mut SourceCode,
    s: &Project,
    def: &ObjectExtension,
) {
    let module_accessor = naming::module_path(&d.get_breadcrumbs());
    let impl_name = naming::impl_type(&def.name);
    let spec_name = naming::runtime_spec_type(&def.name);

    src.import("import typing");
    src.line("@typing.final");
    src.line(&format!(
        "class {impl_name}(runtime_spec.{module_accessor}.{spec_name}):",
    ));
    src.indent();

    let mut pass = true;

    for field in def.iter_fields() {
        if let Some(resolve) = s.resolve_field_resolve(field) {
            pass = false;
            render_field_resolver(d, src, &resolve);
            src.newline();
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
    s: &Project,
    def: &InterfaceExtension,
) {
    let module_accessor = naming::module_path(&d.get_breadcrumbs());
    let impl_name = naming::impl_type(&def.name);
    let spec_name = naming::runtime_spec_type(&def.name);

    src.import("import typing");
    src.line("@typing.final");
    src.line(&format!(
        "class {impl_name}(runtime_spec.{module_accessor}.{spec_name}):",
    ));
    src.indent();
    let mut pass = true;

    for field in def.iter_fields() {
        if let Some(resolve) = s.resolve_field_resolve(field) {
            pass = false;
            render_field_resolver(d, src, &resolve);
            src.newline();
        }
    }

    if pass {
        src.line("pass");
    }
    src.dedent();
}

fn render_field_resolver(d: &ModuleRef, src: &mut SourceCode, resolve: &Resolve) {
    src.import("import graphql");
    src.import("import typing");
    src.import(&format!(
        "from {}. import source_spec",
        ".".repeat(d.get_depth())
    ));

    let sig = if resolve.sync { "def" } else { "async def" };
    let name = naming::field_name(&resolve.field.name);
    let source_type = naming::source_reference(&resolve.field.type_name);
    let return_type = format_type_expression(Some("source_spec"), &resolve.field.field_type);
    let arguments = format_argument_list(&resolve.field.args);

    src.line(&format!(
        "{sig} {name}(self, obj: {source_type}, info: graphql.GraphQLResolveInfo, {arguments}) -> {return_type}:",
    ));

    src.indent();
    src.line("raise NotImplementedError()");
    src.dedent();
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
