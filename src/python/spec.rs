use super::{
    error::Result, format_named_source, format_type_expression, naming, sourcecode::SourceCode,
};
use crate::schema::{
    Definition, InterfaceDefinition, InterfaceExtension, ObjectDefinition, ObjectExtension,
    Resolve, ScalarDefinition, Project, ModuleRef,
};
use std::{fs::File, path::PathBuf};

pub fn render(outdir: &PathBuf, s: &Project) -> Result<()> {
    let filepath = outdir.join("spec.py");
    let mut file = File::create(filepath)?;

    let mut src = SourceCode::new_generated();

    src.import("from . import source");

    src.line("class Spec:");
    src.indent();
    render_directory(&mut src, ModuleRef::new(s))?;
    src.dedent();

    src.write_to(&mut file)?;
    Ok(())
}

fn render_directory(src: &mut SourceCode, d: ModuleRef) -> Result<()> {
    if d.has_children() {
        for child in d.iter_children() {
            src.line(format!("class {}:", child.get_name()));
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
                    src.line("");
                }
                Definition::InterfaceDefinition(inner) => {
                    render_interface_spec(src, d.schema, inner);
                    src.line("");
                }
                Definition::ScalarDefinition(inner) => {
                    render_scalar_spec(src, d.schema, inner);
                    src.line("");
                }
                Definition::ObjectExtension(inner) => {
                    render_object_ext_spec(src, d.schema, inner);
                    src.line("");
                }
                Definition::InterfaceExtension(inner) => {
                    render_interface_ext_spec(src, d.schema, inner);
                    src.line("");
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
        name = naming::spec_type(&def.name)
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
        name = naming::spec_type(&def.name)
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
        name = naming::spec_type(&def.name)
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
        name = naming::spec_type(&def.name)
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
        name = naming::spec_type(&def.name)
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
    let name = naming::field_name(&resolve.field.name);
    let source_type = format_named_source(&resolve.field.type_name);
    let return_type = format_type_expression(&resolve.field.field_type);
    let sig = if resolve.sync { "def" } else { "async def" };

    src.line(&format!(
        "{sig} {name}(self, obj: {source_type}, info: graphql.GraphQLResolveInfo, **args: typing.Any) -> {return_type}: ...",
    ));
}
