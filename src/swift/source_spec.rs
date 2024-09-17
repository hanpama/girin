use super::{error::Error, naming, source_code::SourceCode};
use crate::schema::{
    Definition, EnumDefinition, InputDefinition, InterfaceDefinition, ObjectDefinition, Project,
    ScalarDefinition, TypeExpression, UnionDefinition,
};
use std::{borrow::Borrow, fs::File, path::PathBuf};

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
        .flat_map(|f| f.get_source_configs());

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
        .flat_map(|f| f.get_source_configs());

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
    src.line(&format!("struct {name}: Decodable {{"));
    src.indent();
    for field in s.collect_input_fields(&def.name) {
        let field_name = naming::field_name(&field.name);
        let field_type = format_type_expression(s, &field.field_type);
        src.line(&format!(
            "let {field_name}: (value: {field_type}, isSet: Bool)"
        ));
    }
    src.line("init(from decoder: Decoder) throws {");
    src.indent();
    src.line("let container = try decoder.container(keyedBy: CodingKeys.self)");
    for field in s.collect_input_fields(&def.name) {
        let field_name = naming::field_name(&field.name);
        let field_type = format_type_expression(s, &field.field_type);
        src.line(&format!(
            "{field_name} = (value: try container.decode({field_type}.self, forKey: .{field_name}), isSet: container.contains(.{field_name}))"
        ));
    }
    src.dedent();
    src.line("}");

    src.line("enum CodingKeys: String, CodingKey {");
    src.indent();
    for field in s.collect_input_fields(&def.name) {
        let field_name = naming::field_name(&field.name);
        src.line(&format!("case {field_name}"));
    }
    src.dedent();
    src.line("}");

    src.dedent();
    src.line("}");
}

fn render_enum_source(src: &mut SourceCode, s: &Project, def: &EnumDefinition) {
    let name = naming::source_spec(&def.name);
    src.line(&format!("enum {name}: String, Codable {{"));
    src.indent();
    for value in s.collect_enum_values(&def.name) {
        naming::enum_value_name(&value.name);
        src.line(&format!("case {}", value.name));
    }
    src.dedent();
    src.line("}");
}

fn render_scalar_source(src: &mut SourceCode, s: &Project, def: &ScalarDefinition) {
    println!("render_scalar_source, {:?}", def);
    let alias = def.type_aliases.get("swift");
    let name: String = naming::source_spec(&def.name);
    if let Some(alias) = alias {
        let alias = format_type_alias(src, alias.clone());
        src.line(&format!("typealias {name} = {alias}"));
    } else {
        src.line(&format!("typealias {name} = String"));
    }
}

fn render_union_source(src: &mut SourceCode, s: &Project, def: &UnionDefinition) {
    let name = naming::source_spec(&def.name);
    src.line(&format!("protocol {name} {{}}",));
}

fn format_type_alias(src: &mut SourceCode, expr: String) -> String {
    if expr.contains(".") {
        src.import(expr.split(".").nth(0).unwrap());
    }
    return expr;
}

pub fn format_type_expression(s: &Project, expr: &TypeExpression) -> String {
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
        "ID" => return "String".to_owned(),
        _ => {}
    }
    naming::source_spec(name)
}
