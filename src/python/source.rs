use super::{error::Error, naming, sourcecode::SourceCode};
use crate::schema::{TypeDefinition, Enum, Input, Interface, Object, Scalar, Schema, TypeExpression};
use std::{borrow::Borrow, fs::File, path::PathBuf};

pub fn render_source_defintiion(outdir: &PathBuf, s: &Schema) -> Result<(), Error> {
    let outfile = outdir.join("source.py");

    let mut file = File::create(outfile)?;
    let mut src = SourceCode::new();

    for def in s.iter_all_definitions() {
        match def {
            TypeDefinition::Object(inner) => {
                render_object_source(&mut src, s, inner);
                src.line("");
                src.line("");
            }
            TypeDefinition::Interface(inner) => {
                render_interface_source(&mut src, s, inner);
                src.line("");
                src.line("");
            }
            TypeDefinition::Input(inner) => {
                render_input_source(&mut src, s, inner);
                src.line("");
                src.line("");
            }
            TypeDefinition::Enum(inner) => {
                render_enum_source(&mut src, s, inner);
                src.line("");
                src.line("");
            }
            TypeDefinition::Scalar(inner) => {
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

fn render_object_source(src: &mut SourceCode, s: &Schema, def: &Object) {
    src.import_third("typing");

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

    for conf in s.collect_object_source_configs(def) {
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

fn render_interface_source(src: &mut SourceCode, s: &Schema, def: &Interface) {
    src.import_third("typing");

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
    for conf in s.collect_interface_source_configs(def) {
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

fn render_input_source(src: &mut SourceCode, s: &Schema, def: &Input) {
    src.import_third("typing");

    src.line(&format!("class {name}:", name = naming::input_source(def)));
    src.indent();

    let mut pass = true;
    for conf in s.collect_input_source_configs(def) {
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

fn render_enum_source(src: &mut SourceCode, s: &Schema, def: &Enum) {
    src.import_third("typing");

    src.line(&format!(
        "{name} = typing.Literal[",
        name = naming::enum_source(def)
    ));
    src.indent();
    for value in s.collect_enum_values(def) {
        src.line(&format!("\"{}\",", value.name));
    }
    src.dedent();
    src.line("]");
}

fn render_scalar_source(src: &mut SourceCode, s: &Schema, def: &Scalar) {
    src.import_third("typing");

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
        return src.import_third(&expr);
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

    let definition = s.get_definition(name);
    if definition.is_none() {
        return "typing.Any".to_owned();
    }
    match definition.unwrap() {
        TypeDefinition::Object(inner) => naming::object_source(inner),
        TypeDefinition::Interface(inner) => naming::interface_source(inner),
        TypeDefinition::Scalar(inner) => naming::scalar_source(inner),
        _ => unimplemented!(),
    }
}
