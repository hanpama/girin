use super::{error::Error, naming, source_code::SourceCode, type_expr};
use crate::schema::{
    Definition, EnumDefinition, InputDefinition, InterfaceDefinition, ObjectDefinition, Project,
    ScalarDefinition, TypeExpression, UnionDefinition,
};
use std::{borrow::Borrow, fs::File, path::PathBuf};

pub fn render(outdir: &PathBuf, s: &Project) -> Result<(), Error> {
    let outfile = outdir.join("source_spec.py");

    let mut file = File::create(outfile)?;
    let mut src = SourceCode::new_generated();

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

    src.write_to(&mut file)?;

    Ok(())
}

fn render_object_source(src: &mut SourceCode, s: &Project, def: &ObjectDefinition) {
    src.import("import typing");

    let mut superclasses = vec!["typing.Protocol".to_owned()];
    for interface in &def.interfaces {
        superclasses.push(naming::source(interface));
    }
    superclasses.reverse();

    src.line(&format!(
        "class {name}({superclasses}):",
        name = naming::source(&def.name),
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
            "{name}: \"{type}\"",
            name = naming::field_name(&conf.name),
            type = type_expr::format_type_expression(None, &conf.type_)
        ));
    }

    if pass {
        src.line("pass");
    }

    src.dedent();
}

fn render_interface_source(src: &mut SourceCode, s: &Project, def: &InterfaceDefinition) {
    src.import("import typing");

    let mut superclasses = vec!["typing.Protocol".to_owned()];
    for interface in &def.interfaces {
        superclasses.push(naming::source(interface));
    }
    superclasses.reverse();

    src.line(&format!(
        "class {name}({superclasses}):",
        name = naming::source(&def.name),
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
            "{name}: \"{type}\"",
            name = naming::field_name(&conf.name),
            type = type_expr::format_type_expression(None, &conf.type_)
        ));
    }

    if pass {
        src.line("pass");
    }

    src.dedent();
}

fn render_input_source(src: &mut SourceCode, s: &Project, def: &InputDefinition) {
    src.import("import typing");

    src.line(&format!(
        "class {name}(typing.TypedDict):",
        name = naming::source(&def.name)
    ));
    src.indent();

    let mut pass = true;
    for conf in s.collect_input_fields(&def.name) {
        pass = false;
        let name = naming::field_name(&conf.name);
        let type_expr = type_expr::format_type_expression(None, &conf.field_type);
        let field = if conf.field_type.is_nullable() {
            format!("{name}: \"typing.NotRequired[{type_expr}]\"")
        } else {
            format!("{name}: \"{type_expr}\"")
        };
        src.line(field);
    }

    if pass {
        src.line("pass");
    }

    src.dedent();
}

fn render_enum_source(src: &mut SourceCode, s: &Project, def: &EnumDefinition) {
    src.import("import typing");

    src.line(&format!(
        "{name} = typing.Literal[",
        name = naming::source(&def.name)
    ));
    src.indent();
    for value in s.collect_enum_values(&def.name) {
        src.line(&format!("\"{}\",", value.name));
    }
    src.dedent();
    src.line("]");
}

fn render_scalar_source(src: &mut SourceCode, s: &Project, def: &ScalarDefinition) {
    src.import("import typing");

    let alias = def.type_aliases.get("python");
    if let Some(alias) = alias {
        // Import
        let alias = format_type_alias(src, alias.clone());
        src.line(&format!(
            "{name} = {alias}",
            name = naming::source(&def.name),
            alias = alias,
        ));
    } else {
        src.line(&format!(
            "{name} = typing.Any",
            name = naming::source(&def.name)
        ));
    }
}

fn render_union_source(src: &mut SourceCode, s: &Project, def: &UnionDefinition) {
    src.import("import typing");

    let mut types = vec![];
    for type_name in s.collect_union_types(&def.name) {
        types.push(format!("\"{}\"", naming::source(type_name)));
    }

    src.line(&format!(
        "{name} = typing.Union[{types}]",
        name = naming::source(&def.name),
        types = types.join(", ")
    ));
}

fn format_type_alias(src: &mut SourceCode, expr: String) -> String {
    if expr.contains(".") {
        todo!();
        return src.import(&expr);
    }
    return expr;
}

// fn format_type_expression(s: &Project, expr: &TypeExpression) -> String {
//     match expr {
//         TypeExpression::NonNullType(inner) => match inner.borrow() {
//             TypeExpression::NamedType(ref name) => format_named_type(name),
//             TypeExpression::ListType(inner) => {
//                 format!("typing.List[{}]", format_type_expression(s, inner.borrow()))
//             }
//             _ => unreachable!(),
//         },
//         TypeExpression::NamedType(name) => {
//             format!("typing.Optional[{}]", format_named_type(name))
//         }
//         TypeExpression::ListType(inner) => {
//             format!(
//                 "typing.Optional[typing.List[{}]]",
//                 format_type_expression(s, inner.borrow())
//             )
//         }
//     }
// }

// fn format_named_type(name: &str) -> String {
//     match name {
//         "String" => return "str".to_owned(),
//         "Int" => return "int".to_owned(),
//         "Float" => return "float".to_owned(),
//         "Boolean" => return "bool".to_owned(),
//         "ID" => return "typing.Any".to_owned(),
//         _ => {}
//     }
//     naming::source(name)
// }
