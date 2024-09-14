use super::{error::Result, naming, source_code::SourceCode, type_expr};
use crate::schema::{
    Definition, InputValue, InterfaceDefinition, InterfaceExtension, ModuleRef, ObjectDefinition,
    ObjectExtension, Project, Resolve, ScalarDefinition,
};
use std::{fs::File, path::PathBuf};

pub fn render(outdir: &PathBuf, s: &Project) -> Result<()> {
    let filepath = outdir.join("RuntimeSpec.swift");
    let mut file = File::create(filepath)?;

    let mut src = SourceCode::new();

    src.line("struct RuntimeSpec {");
    src.indent();
    render_directory(&mut src, ModuleRef::new(s))?;
    src.dedent();
    src.line("}");

    src.write_to(&mut file)?;
    Ok(())
}

fn render_directory(src: &mut SourceCode, d: ModuleRef) -> Result<()> {
    if d.has_children() {
        for child in d.iter_children() {
            let name = naming::module_name(child.get_name());
            src.line(format!("struct {name} {{"));
            src.indent();
            render_directory(src, child)?;
            src.dedent();
            src.line("}");
        }
    }
    if d.has_definition() {
        for type_ in d.iter_definitions() {
            match type_ {
                Definition::ObjectDefinition(inner) => {
                    render_object_spec(src, d.schema, inner);
                }
                Definition::InterfaceDefinition(inner) => {
                    render_interface_spec(src, d.schema, inner);
                }
                Definition::ScalarDefinition(inner) => {
                    render_scalar_spec(src, d.schema, inner);
                }
                Definition::ObjectExtension(inner) => {
                    render_object_ext_spec(src, d.schema, inner);
                }
                Definition::InterfaceExtension(inner) => {
                    render_interface_ext_spec(src, d.schema, inner);
                }
                _ => { /* noop */ }
            }
        }
    }
    if d.has_children() {
        for child in d.iter_children() {
            let name = naming::module_name(child.get_name());
            src.line(format!("var {name}: {name}"));
        }
    }
    if d.has_definition() {
        for type_ in d.iter_definitions() {
            match type_ {
                Definition::ObjectDefinition(inner) => {
                    let name = naming::runtime_spec(&inner.name);
                    src.line(format!("var {name}: {name}"));
                }
                Definition::InterfaceDefinition(inner) => {
                    let name = naming::runtime_spec(&inner.name);
                    src.line(format!("var {name}: {name}"));
                }
                Definition::ScalarDefinition(inner) => {
                    let name = naming::runtime_spec(&inner.name);
                    src.line(format!("var {name}: {name}"));
                }
                Definition::ObjectExtension(inner) => {
                    let name = naming::runtime_spec(&inner.name);
                    src.line(format!("var {name}: {name}"));
                }
                Definition::InterfaceExtension(inner) => {
                    let name = naming::runtime_spec(&inner.name);
                    src.line(format!("var {name}: {name}"));
                }
                _ => { /* noop */ }
            }
        }
    }

    Ok(())
}

fn render_object_spec(src: &mut SourceCode, s: &Project, def: &ObjectDefinition) {
    src.line(&format!(
        "struct {name} {{",
        name = naming::runtime_spec(&def.name)
    ));
    src.indent();

    for field in def.iter_fields() {
        if let Some(resolve) = s.resolve_field_resolve(field) {
            render_field_resolver(src, &resolve);
        }
    }
    src.dedent();
    src.line("}");
}

fn render_interface_spec(src: &mut SourceCode, s: &Project, def: &InterfaceDefinition) {
    src.line(&format!(
        "struct {name} {{",
        name = naming::runtime_spec(&def.name)
    ));
    src.indent();

    for field in def.iter_fields() {
        if let Some(resolve) = s.resolve_field_resolve(field) {
            render_field_resolver(src, &resolve);
        }
    }

    src.dedent();
    src.line("}");
}

fn render_scalar_spec(src: &mut SourceCode, s: &Project, def: &ScalarDefinition) {
    src.import("GraphQL");

    src.line(&format!(
        "struct {name} {{",
        name = naming::runtime_spec(&def.name)
    ));
    src.indent();

    src.line("var serialize: (_ value: Any) throws -> GraphQL.Map");
    src.line("var parseValue: (_ value: GraphQL.Map) throws -> GraphQL.Map");
    src.line("var parseLiteral: (_ value: GraphQL.Value) throws -> GraphQL.Map");

    src.dedent();
    src.line("}");
}

fn render_object_ext_spec(src: &mut SourceCode, s: &Project, def: &ObjectExtension) {
    let name = naming::runtime_spec(&def.name);
    src.line(&format!("struct {name} {{"));
    src.indent();

    for field in def.iter_fields() {
        if let Some(resolve) = s.resolve_field_resolve(field) {
            render_field_resolver(src, &resolve);
        }
    }

    src.dedent();
    src.line("}");
}

fn render_interface_ext_spec(src: &mut SourceCode, s: &Project, def: &InterfaceExtension) {
    src.line(&format!(
        "struct {name} {{",
        name = naming::runtime_spec(&def.name)
    ));
    src.indent();

    for field in def.iter_fields() {
        if let Some(resolve) = s.resolve_field_resolve(field) {
            render_field_resolver(src, &resolve);
        }
    }
    src.dedent();
    src.line("}");
}

fn render_field_resolver(src: &mut SourceCode, resolve: &Resolve) {
    let sig = if resolve.sync {
        "throws"
    } else {
        "async throws"
    };
    let name = naming::field_name(&resolve.field.name);
    let source_type = type_expr::format_named_type(Some("SourceSpec"), &resolve.field.type_name);
    let return_type =
        type_expr::format_type_expression(Some("SourceSpec"), &resolve.field.field_type);
    let arguments = format_argument_list(&resolve.field.args);

    let arguments = if arguments.is_empty() {
        "".to_string()
    } else {
        format!("{}, ", arguments)
    };

    src.line(&format!(
        "var {name}: (_: (source: {source_type}, {arguments}context: Any, info: GraphQL.GraphQLResolveInfo)) {sig} -> {return_type}",
    ));
}

fn format_argument_list(args: &Vec<InputValue>) -> String {
    let mut els = args
        .iter()
        .map(|input| {
            let name = naming::field_name(&input.name);
            let expr = type_expr::format_type_expression(Some("SourceSpec"), &input.field_type);
            format!("{name}: {expr}")
        })
        .collect::<Vec<_>>();
    if args.len() == 1 {
        els.push("_: ()".to_owned());
    }

    format!("args: ({})", els.join(", "))
}
