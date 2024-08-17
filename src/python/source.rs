use super::{error::Error, naming, sourcecode::SourceCode};
use crate::schema::{
    Definition, EnumDefinition, InputDefinition, InterfaceDefinition, ObjectDefinition,
    ScalarDefinition, Schema, TypeExpression,
};
use std::{borrow::Borrow, fs::File, path::PathBuf};

pub fn render_source_defintiion(outdir: &PathBuf, s: &Schema) -> Result<(), Error> {
    let outfile = outdir.join("source.py");

    let mut file = File::create(outfile)?;
    let mut src = SourceCode::new();

    for def in s.iter_definitions() {
        match def {
            Definition::ObjectDefinition(inner) => {
                render_object_source(&mut src, s, inner);
                src.line("");
                src.line("");
            }
            Definition::InterfaceDefinition(inner) => {
                render_interface_source(&mut src, s, inner);
                src.line("");
                src.line("");
            }
            Definition::InputDefinition(inner) => {
                render_input_source(&mut src, s, inner);
                src.line("");
                src.line("");
            }
            Definition::EnumDefinition(inner) => {
                render_enum_source(&mut src, s, inner);
                src.line("");
                src.line("");
            }
            Definition::ScalarDefinition(inner) => {
                render_scalar_source(&mut src, s, inner);
                src.line("");
                src.line("");
            }
            _ => {}
        }
    }

    src.write_to(&mut file)?;

    Ok(())
}

fn render_object_source(src: &mut SourceCode, s: &Schema, def: &ObjectDefinition) {
    src.import_std("typing");

    let mut superclasses = vec!["typing.Protocol".to_owned()];
    for interface in &def.interfaces {
        superclasses.push(naming::source(interface));
    }
    superclasses.reverse();

    src.line(&format!(
        "class {name}({superclasses}):",
        name = naming::object_source(def),
        superclasses = superclasses.join(", ")
    ));
    src.indent();

    let mut pass = true;

    let source_configs = s
        .collect_fields(&def.name)
        .into_iter()
        .flat_map(|f| f.collect_source_configs());

    for conf in source_configs {
        pass = false;
        src.line(&format!(
            "{name}: {type}",
            name = conf.name,
            type = format_type_expression(s, &conf.type_)
        ));
    }

    if pass {
        src.line("pass");
    }

    src.dedent();
}

fn render_interface_source(src: &mut SourceCode, s: &Schema, def: &InterfaceDefinition) {
    src.import_std("typing");

    let mut superclasses = vec!["typing.Protocol".to_owned()];
    for interface in &def.interfaces {
        superclasses.push(naming::source(interface));
    }
    superclasses.reverse();

    src.line(&format!(
        "class {name}({superclasses}):",
        name = naming::interface_source(def),
        superclasses = superclasses.join(", ")
    ));
    src.indent();

    let mut pass = true;

    let source_configs = s
        .collect_fields(&def.name)
        .into_iter()
        .flat_map(|f| f.collect_source_configs());

    for conf in source_configs {
        pass = false;
        src.line(&format!(
            "{name}: {type}",
            name = conf.name,
            type = format_type_expression(s, &conf.type_)
        ));
    }

    if pass {
        src.line("pass");
    }

    src.dedent();
}

fn render_input_source(src: &mut SourceCode, s: &Schema, def: &InputDefinition) {
    src.import_std("typing");

    src.line(&format!("class {name}:", name = naming::input_source(def)));
    src.indent();

    let mut pass = true;
    for conf in s.collect_input_fields(&def.name) {
        pass = false;
        src.line(&format!(
            "{name}: {type}",
            name = &conf.name,
            type = format_type_expression(s, &conf.field_type)
        ));
    }

    if pass {
        src.line("pass");
    }

    src.dedent();
}

fn render_enum_source(src: &mut SourceCode, s: &Schema, def: &EnumDefinition) {
    src.import_std("typing");

    src.line(&format!(
        "{name} = typing.Literal[",
        name = naming::enum_source(def)
    ));
    src.indent();
    for value in s.collect_enum_values(&def.name) {
        src.line(&format!("\"{}\",", value.name));
    }
    src.dedent();
    src.line("]");
}

fn render_scalar_source(src: &mut SourceCode, s: &Schema, def: &ScalarDefinition) {
    src.import_std("typing");

    let alias = def.type_aliases.get("python");
    if let Some(alias) = alias {
        // Import
        let alias = format_type_alias(src, alias.clone());
        src.line(&format!(
            "{name} = {alias}",
            name = naming::scalar_source(def),
            alias = alias,
        ));
    } else {
        src.line(&format!(
            "{name} = typing.Any",
            name = naming::scalar_source(def)
        ));
    }
}

fn format_type_expression(s: &Schema, expr: &TypeExpression) -> String {
    format!("\"{}\"", _format_type_expression(s, expr))
}

fn _format_type_expression(s: &Schema, expr: &TypeExpression) -> String {
    match expr {
        TypeExpression::NonNullType(inner) => match inner.borrow() {
            TypeExpression::NamedType(ref name) => format_named_type(s, name),
            TypeExpression::ListType(inner) => {
                format!("typing.List[{}]", format_type_expression(s, inner.borrow()))
            }
            _ => unreachable!(),
        },
        TypeExpression::NamedType(name) => {
            format!("typing.Optional[{}]", format_named_type(s, name))
        }
        TypeExpression::ListType(inner) => {
            format!(
                "typing.Optional[typing.List[{}]]",
                format_type_expression(s, inner.borrow())
            )
        }
    }
}

fn format_type_alias(src: &mut SourceCode, expr: String) -> String {
    if expr.contains(".") {
        return src.import_first(&expr);
    }
    return expr;
}

fn format_named_type(s: &Schema, name: &str) -> String {
    match name {
        "String" => return "str".to_owned(),
        "Int" => return "int".to_owned(),
        "Float" => return "float".to_owned(),
        "Boolean" => return "bool".to_owned(),
        "ID" => return "typing.Any".to_owned(),
        _ => {}
    }
    naming::source(s.get_definition(name).get_name())
}
