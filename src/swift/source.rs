use super::{error::Error, naming, sourcecode::SourceCode};
use crate::schema::{
    Definition, EnumDefinition, InputDefinition, InterfaceDefinition, ObjectDefinition, Project,
    ScalarDefinition, TypeExpression, UnionDefinition,
};
use std::{borrow::Borrow, fs::File, io, path::PathBuf};

pub fn render(outdir: &PathBuf, s: &Project) -> Result<(), Error> {
    let outfile = outdir.join("SourceSpec.swift");

    let mut file = File::create(outfile)?;
    let mut src = SourceCode::new();

    src.line("struct SourceSpec {");
    src.indent();

    for def in s.iter_type_definitions() {
        match def {
            Definition::ObjectDefinition(inner) => {
                render_object_source(&mut src, s, inner);
                src.newline();
            }
            Definition::InterfaceDefinition(inner) => {
                render_interface_source(&mut src, s, inner);
                src.newline();
            }
            Definition::InputDefinition(inner) => {
                render_input_source(&mut src, s, inner);
                src.newline();
                src.newline();
            }
            Definition::EnumDefinition(inner) => {
                render_enum_source(&mut src, s, inner);
                src.newline();
            }
            Definition::ScalarDefinition(inner) => {
                render_scalar_source(&mut src, s, inner);
                src.newline();
            }
            Definition::UnionDefinition(inner) => {
                render_union_source(&mut src, s, inner);
                src.newline();
            }
            _ => {}
        }
    }

    src.dedent();
    src.line("}");

    src.write_to(&mut file)?;

    Ok(())
}

fn render_object_source(src: &mut SourceCode, s: &Project, def: &ObjectDefinition) {
    let name = naming::source_spec(&def.name);
    let mut supertypes = Vec::new();

    for interface in s.collect_interfaces(&def.name) {
        supertypes.push(naming::source_spec(interface));
    }
    for union in s.collect_object_unions(&def.name) {
        supertypes.push(naming::source_spec(union));
    }
    let supertypes = if supertypes.is_empty() {
        "".to_owned()
    } else {
        format!(": {}", supertypes.join(", "))
    };

    src.line(&format!("protocol {name}{supertypes} {{",));
    src.indent();

    let source_configs = s
        .collect_fields(&def.name)
        .into_iter()
        .flat_map(|f| f.collect_source_configs());

    for conf in source_configs {
        src.line(&format!(
            "var {name}: {type} {{ get }}",
            name = naming::field_name(&conf.name),
            type = format_type_expression(s, &conf.type_)
        ));
    }

    src.dedent();
    src.line("}");
}

fn render_interface_source(src: &mut SourceCode, s: &Project, def: &InterfaceDefinition) {
    let name = naming::source_spec(&def.name);
    src.line(&format!("protocol {name} {{",));
    src.indent();

    let source_configs = s
        .collect_fields(&def.name)
        .into_iter()
        .flat_map(|f| f.collect_source_configs());

    for conf in source_configs {
        src.line(&format!(
            "var {name}: {type} {{ get }}",
            name = naming::field_name(&conf.name),
            type = format_type_expression(s, &conf.type_)
        ));
    }

    src.dedent();
    src.line("}");
}

fn render_input_source(src: &mut SourceCode, s: &Project, def: &InputDefinition) {
    let name = naming::source_spec(&def.name);
    src.line(&format!("struct {name} {{"));
    src.indent();
    for field in s.collect_input_fields(&def.name) {
        let field_name = &field.name;
        let field_type = format_type_expression(s, &field.field_type);
        src.line(&format!(
            "let {field_name}: (value: {field_type}, isSet: Bool)"
        ));
    }
    src.dedent();
    src.line("}");
}

fn render_enum_source(src: &mut SourceCode, s: &Project, def: &EnumDefinition) {
    let name = naming::source_spec(&def.name);
    src.line(&format!("enum {name} {{"));
    src.indent();
    for value in s.collect_enum_values(&def.name) {
        src.line(&format!("case {}", value.name));
    }
    src.dedent();
    src.line("}");
}

fn render_scalar_source(src: &mut SourceCode, s: &Project, def: &ScalarDefinition) {
    let alias = def.type_aliases.get("swift");
    if let Some(alias) = alias {
        // Import
        let alias = format_type_alias(src, alias.clone());
        src.line(&format!(
            "{name} = {alias}",
            name = naming::source_spec(&def.name),
            alias = alias,
        ));
    } else {
        src.line(&format!(
            "typealias {name} = Any",
            name = naming::source_spec(&def.name)
        ));
    }
}

fn render_union_source(src: &mut SourceCode, s: &Project, def: &UnionDefinition) {
    let name = naming::source_spec(&def.name);
    src.line(&format!("protocol {name} {{}}",));
    // for type_name in s.collect_union_types(&def.name) {
    //     let possible_source_name = naming::source(type_name);
    //     src.line(format!("extension {possible_source_name}: {name} {{}}"));
    // }
}

fn format_type_alias(src: &mut SourceCode, expr: String) -> String {
    if expr.contains(".") {
        todo!();
        // return src.import(&expr);
    }
    return expr;
}

fn format_type_expression(s: &Project, expr: &TypeExpression) -> String {
    match expr {
        TypeExpression::NonNullType(inner) => match inner.borrow() {
            TypeExpression::NamedType(ref name) => format_named_type(name),
            TypeExpression::ListType(inner) => {
                format!("[{}]", format_type_expression(s, inner.borrow()))
            }
            _ => unreachable!(),
        },
        TypeExpression::NamedType(name) => {
            format!("{}?", format_named_type(name))
        }
        TypeExpression::ListType(inner) => {
            format!("[{}]?", format_type_expression(s, inner.borrow()))
        }
    }
}

fn format_named_type(name: &str) -> String {
    match name {
        "String" => return "String".to_owned(),
        "Int" => return "Int".to_owned(),
        "Float" => return "Float".to_owned(),
        "Boolean" => return "Bool".to_owned(),
        "ID" => return "Any".to_owned(),
        _ => {}
    }
    naming::source_spec(name)
}
