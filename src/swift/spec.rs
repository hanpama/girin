use super::{error::Result, naming, sourcecode::SourceCode, type_expr};
use crate::schema::{
    Definition, InputValue, InterfaceDefinition, InterfaceExtension, ModuleRef, ObjectDefinition,
    ObjectExtension, Project, Resolve, ScalarDefinition,
};
use std::{fs::File, path::PathBuf};

pub fn render(outdir: &PathBuf, s: &Project) -> Result<()> {
    let filepath = outdir.join("ResolverSpec.swift");
    let mut file = File::create(filepath)?;

    let mut src = SourceCode::new();

    src.line("struct ResolverSpec {");
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
            src.line(format!("struct {} {{", child.get_name()));
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

    Ok(())
}

fn render_object_spec(src: &mut SourceCode, s: &Project, def: &ObjectDefinition) {
    src.line(&format!(
        "protocol {name} {{",
        name = naming::resolver_spec(&def.name)
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
        "protocol {name} {{",
        name = naming::resolver_spec(&def.name)
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
        "protocol {name} {{",
        name = naming::resolver_spec(&def.name)
    ));
    src.indent();

    src.line("func serialize(_ value: Any) throws -> GraphQL.Map");
    src.line("func parseValue(_ value: GraphQL.Map) throws -> GraphQL.Map");
    src.line("func parseLiteral(_ value: GraphQL.Value) throws -> GraphQL.Map");

    src.dedent();
    src.line("}");
}

fn render_object_ext_spec(src: &mut SourceCode, s: &Project, def: &ObjectExtension) {
    let name = naming::resolver_spec(&def.name);
    src.line(&format!("protocol {name} {{"));
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
        "protocol {name} {{",
        name = naming::resolver_spec(&def.name)
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
        "async throws"
    } else {
        "throws"
    };
    let name = naming::field_name(&resolve.field.name);
    let source_type = type_expr::format_named_source(&resolve.field.type_name);
    let return_type = type_expr::format_type_expression(&resolve.field.field_type);
    let arguments = format_argument_list(&resolve.field.args);

    let arguments = if arguments.is_empty() {
        "".to_string()
    } else {
        format!(", {}", arguments.join(", "))
    };

    src.line(&format!(
        "func {name}(obj: {source_type}, info: GraphQL.GraphQLResolveInfo{arguments}) {sig} -> {return_type}",
    ));
}

fn format_argument_list(args: &Vec<InputValue>) -> Vec<String> {
    args.iter()
        .map(|input| {
            let name = naming::field_name(&input.name);
            let expr = type_expr::format_type_expression(&input.field_type);
            format!("{name}: {expr}")
        })
        .collect::<Vec<_>>()
}
