use super::{error::Result, format_type_expression, naming, source_code::SourceCode};
use crate::schema::{
    Definition, InputValue, InterfaceDefinition, InterfaceExtension, ModuleRef, ObjectDefinition,
    ObjectExtension, Project, Resolve, ScalarDefinition,
};
use std::{fs::File, path::PathBuf};

pub fn render(outdir: &PathBuf, s: &Project) -> Result<()> {
    let filepath = outdir.join("runtime_spec.py");
    let mut file = File::create(filepath)?;

    let mut src = SourceCode::new_generated();

    src.import("from . import source_spec");

    // src.line("class ResolverSpec:");
    // src.indent();
    render_directory(&mut src, ModuleRef::new(s))?;
    // src.dedent();

    src.write_to(&mut file)?;
    Ok(())
}

fn render_directory(src: &mut SourceCode, d: ModuleRef) -> Result<()> {
    if d.has_children() {
        for child in d.iter_children() {
            src.line(format!(
                "class {name}:",
                name = naming::module_name(&child.get_name())
            ));
            src.indent();
            render_directory(src, child)?;
            src.dedent();
        }
    }
    if d.has_definition() {
        for type_ in d.iter_definitions() {
            match type_ {
                Definition::ObjectDefinition(inner) => {
                    render_object_spec(src, d.schema, inner);
                    src.newline();
                }
                Definition::InterfaceDefinition(inner) => {
                    render_interface_spec(src, d.schema, inner);
                    src.newline();
                }
                Definition::ScalarDefinition(inner) => {
                    render_scalar_spec(src, d.schema, inner);
                    src.newline();
                }
                Definition::ObjectExtension(inner) => {
                    render_object_ext_spec(src, d.schema, inner);
                    src.newline();
                }
                Definition::InterfaceExtension(inner) => {
                    render_interface_ext_spec(src, d.schema, inner);
                    src.newline();
                }
                _ => { /* noop */ }
            }
        }
    }

    Ok(())
}

fn render_object_spec(src: &mut SourceCode, s: &Project, def: &ObjectDefinition) {
    src.line(&format!(
        "class {name}(typing.Protocol):",
        name = naming::runtime_spec_type(&def.name)
    ));
    src.indent();

    let mut pass = true;

    for field in def.iter_fields() {
        if let Some(resolve) = s.resolve_field_resolve(field) {
            pass = false;
            render_field_resolver(src, &resolve);
        }
    }

    if pass {
        src.line("pass");
    }
    src.dedent();
}

fn render_interface_spec(src: &mut SourceCode, s: &Project, def: &InterfaceDefinition) {
    src.line(&format!(
        "class {name}(typing.Protocol):",
        name = naming::runtime_spec_type(&def.name)
    ));
    src.indent();

    let mut pass = true;

    for field in def.iter_fields() {
        if let Some(resolve) = s.resolve_field_resolve(field) {
            pass = false;
            render_field_resolver(src, &resolve);
        }
    }

    if pass {
        src.line("pass");
    }
    src.dedent();
}

fn render_scalar_spec(src: &mut SourceCode, s: &Project, def: &ScalarDefinition) {
    src.import("import typing");
    src.import("import graphql");
    src.line(&format!(
        "class {name}(typing.Protocol):",
        name = naming::runtime_spec_type(&def.name)
    ));
    src.indent();

    src.line("def serialize(self, value: typing.Any) -> typing.Any: ...");
    src.line("def parse_value(self, value: typing.Any) -> typing.Any: ...");
    src.line("def parse_literal(self, node: graphql.ValueNode, variables: typing.Any) -> typing.Any: ...");

    src.dedent();
}

fn render_object_ext_spec(src: &mut SourceCode, s: &Project, def: &ObjectExtension) {
    src.line(&format!(
        "class {name}(typing.Protocol):",
        name = naming::runtime_spec_type(&def.name)
    ));
    src.indent();

    let mut pass = true;

    for field in def.iter_fields() {
        if let Some(resolve) = s.resolve_field_resolve(field) {
            pass = false;
            render_field_resolver(src, &resolve);
        }
    }

    if pass {
        src.line("pass");
    }
    src.dedent();
}

fn render_interface_ext_spec(src: &mut SourceCode, s: &Project, def: &InterfaceExtension) {
    src.line(&format!(
        "class {name}(typing.Protocol):",
        name = naming::runtime_spec_type(&def.name)
    ));
    src.indent();
    let mut pass = true;

    for field in def.iter_fields() {
        if let Some(resolve) = s.resolve_field_resolve(field) {
            pass = false;
            render_field_resolver(src, &resolve);
        }
    }

    if pass {
        src.line("pass");
    }
    src.dedent();
}

fn render_field_resolver(src: &mut SourceCode, resolve: &Resolve) {
    let sig = if resolve.sync { "def" } else { "async def" };
    let name = naming::field_name(&resolve.field.name);
    let source_type = naming::source_reference(&resolve.field.type_name);
    let return_type = format_type_expression(Some("source_spec"), &resolve.field.field_type);
    let arguments = format_argument_list(&resolve.field.args);

    src.line(&format!(
        "{sig} {name}(self, obj: {source_type}, info: graphql.GraphQLResolveInfo, {arguments}) -> {return_type}: ...",
    ));
}

fn format_argument_list(args: &Vec<InputValue>) -> String {
    args.iter()
        .map(|input| {
            let name = naming::field_name(&input.name);
            let expr = format_type_expression(Some("source_spec"), &input.field_type);
            format!("{name}: {expr}")
        })
        .collect::<Vec<_>>()
        .join(", ")
}
