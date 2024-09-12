use super::{error::Error, naming, sourcecode::SourceCode};
use crate::schema::{
    Definition, EnumDefinition, EnumValue, Field, InputDefinition, InputValue, InterfaceDefinition,
    ModuleRef, ObjectDefinition, Project, ScalarDefinition, TypeExpression, UnionDefinition, Value,
};
use std::{fs::File, path::PathBuf};

pub fn render(outdir: &PathBuf, s: &Project) -> Result<(), Error> {
    let outfile = outdir.join("builder.swift");

    let mut file = File::create(outfile)?;
    let mut src = SourceCode::new();

    src.import("GraphQL");

    src.line("func buildSchema(config: BuilderConfig) throws -> GraphQLSchema {");
    src.indent();

    // src.line("return GraphQLSchema(");
    // src.indent();

    // if let Some(query) = &s.get_query() {
    //     src.line(format!("query: {},", naming::type_instance(query)));
    // }
    // if let Some(mutation) = &s.get_mutation() {
    //     src.line(format!("mutation: {},", naming::type_instance(mutation)));
    // }
    // if let Some(subscription) = &s.get_subscription() {
    //     src.line(format!(
    //         "subscription: {},",
    //         naming::type_instance(subscription)
    //     ));
    // }
    // src.line("types=[");
    // src.indent();
    // for def in s.iter_type_definitions() {
    //     src.line(format!(
    //         "{},",
    //         naming::type_instance(def.get_definition_name().unwrap())
    //     ));
    // }
    // src.dedent();
    // src.line("],");

    // src.dedent();
    // src.line(")");

    src.dedent();
    src.line("}");

    src.line("fileprivate struct TypeRegistry {");
    src.indent();

    src.line("let config: BuilderConfig");
    for def in s.iter_type_definitions() {
        match def {
            Definition::ObjectDefinition(inner) => render_object_type(&mut src, s, inner),
            Definition::InterfaceDefinition(inner) => render_interface_type(&mut src, s, inner),
            Definition::InputDefinition(inner) => render_input_type(&mut src, s, inner),
            // Definition::ScalarDefinition(inner) => render_scalar_type(&mut src, s, inner),
            // Definition::EnumDefinition(inner) => render_enum_type(&mut src, s, inner),
            // Definition::UnionDefinition(inner) => render_union_type(&mut src, s, inner),
            _ => {}
        }
    }

    src.dedent();
    src.line("}");

    src.write_to(&mut file)?;

    Ok(())
}

fn render_object_type(src: &mut SourceCode, s: &Project, def: &ObjectDefinition) {
    let name = naming::type_instance(&def.name);
    src.line(format!("lazy var {} = try! GraphQLObjectType(", name));
    src.indent();
    src.line(format!("name: \"{}\"", def.name));

    if let Some(description) = &def.description {
        src.append(",");
        src.line(format!("description: {:?}", description));
    }

    src.append(",");
    src.line("fields: [");
    src.indent();
    for field in s.collect_fields(&def.name) {
        render_field(src, s, field)
    }
    src.dedent();
    src.line("]");

    let interfaces = s.collect_interfaces(&def.name);
    if interfaces.len() > 0 {
        src.append(",");
        src.line("interfaces: [");
        src.indent();
        for interface in interfaces {
            src.line(format!("{},", naming::type_instance(interface)));
        }
        src.dedent();
        src.line("]");
    }

    src.dedent();
    src.line(")");
}

fn render_interface_type(src: &mut SourceCode, s: &Project, def: &InterfaceDefinition) {
    let name = naming::type_instance(&def.name);
    src.line(format!("lazy var {} = try! GraphQLInterfaceType(", name));
    src.indent();
    src.line(format!("name: \"{}\"", def.name));

    if let Some(description) = &def.description {
        src.append(",");
        src.line(format!("description: {:?},", description));
    }

    if def.interfaces.len() > 0 {
        src.append(",");
        src.line("interfaces: [");
        src.indent();
        for interface in s.collect_interfaces(&def.name) {
            src.line(format!("{},", naming::type_instance(interface)));
        }
        src.dedent();
        src.line("]");
    }

    src.append(",");
    src.line("fields: [");
    src.indent();
    for field in s.collect_fields(&def.name) {
        render_field(src, s, field)
    }
    src.dedent();
    src.line("]");

    src.dedent();
    src.line(")");
}

fn render_input_type(src: &mut SourceCode, s: &Project, def: &InputDefinition) {
    let name = naming::type_instance(&def.name);
    src.line(format!("lazy var {} = try! GraphQLInputObjectType(", name));
    src.indent();
    src.line(format!("name: \"{}\"", def.name));

    if let Some(description) = &def.description {
        src.append(",");
        src.line(format!("description: {:?},", description));
    }

    src.append(",");
    src.line("fields: [");
    src.indent();
    for field in s.collect_input_fields(&def.name) {
        render_input_field(src, field)
    }
    src.dedent();
    src.line("]");

    src.dedent();
    src.line(")");
}

fn render_scalar_type(src: &mut SourceCode, s: &Project, def: &ScalarDefinition) {
    let name = naming::type_instance(&def.name);
    src.line(format!("{} = GraphQLScalarType(", name));
    src.indent();
    src.line(format!("name=\"{}\",", def.name));

    if let Some(description) = &def.description {
        src.line(format!("description: {:?},", description));
    }

    let config_path =
        format_definition_config_path(s.get_module_ref(&def.position.file), &def.name);
    src.line(format!("serialize: {config_path}.serialize,"));
    src.line(format!("parse_value: {config_path}.parse_value,"));
    src.line(format!("parse_literal: {config_path}.parse_literal,"));

    src.dedent();
    src.line(")");
}

fn render_enum_type(src: &mut SourceCode, s: &Project, def: &EnumDefinition) {
    let name = naming::type_instance(&def.name);
    src.line(format!("{} = GraphQLEnumType(", name));
    src.indent();
    src.line(format!("name=\"{}\",", def.name));

    if let Some(description) = &def.description {
        src.line(format!("description: {:?},", description));
    }

    src.line("values: {");
    src.indent();
    for value in s.collect_enum_values(&def.name) {
        render_enum_value(src, s, value)
    }
    src.dedent();
    src.line("}");

    src.dedent();
    src.line(")");
}

fn render_enum_value(src: &mut SourceCode, s: &Project, def: &EnumValue) {
    src.line(format!("{:?}: GraphQLEnumValue(", def.name));
    src.indent();
    src.line(format!("value: {:?},", &def.name));
    if let Some(description) = &def.description {
        src.line(format!("description: {:?},", description));
    }
    if let Some(deprecation_reason) = &def.deprecation_reason {
        src.line(format!("deprecation_reason: {:?},", deprecation_reason));
    }
    src.dedent();
    src.line("),");
}

fn render_union_type(src: &mut SourceCode, s: &Project, def: &UnionDefinition) {
    let name = naming::type_instance(&def.name);
    src.line(format!("{} = GraphQLUnionType(", name));
    src.indent();
    src.line(format!("name=\"{}\",", def.name));

    if let Some(description) = &def.description {
        src.line(format!("description: {:?},", description));
    }

    src.line("types=lambda: [");
    src.indent();
    for value in s.collect_union_types(&def.name) {
        src.line(format!("{},", naming::type_instance(value)));
    }
    src.dedent();
    src.line("]");

    src.dedent();
    src.line(")");
}

fn render_field(src: &mut SourceCode, s: &Project, def: &Field) {
    src.line(format!("{:?}: .init(", def.name));
    src.indent();

    src.line(format!(
        "type: {},",
        format_type_expression(&def.field_type)
    ));
    if let Some(deprecation_reason) = &def.deprecation_reason {
        src.line(format!("deprecationReason: {:?},", deprecation_reason));
    }
    if let Some(description) = &def.description {
        src.line(format!("description: {:?},", description));
    }

    if !def.args.is_empty() {
        src.line("args: [");
        src.indent();
        for arg in &def.args {
            render_arg(src, arg);
        }
        src.dedent();
        src.line("],");
    }
    if let Some(opt) = s.resolve_field_resolve(def) {
        let module_ref = s.get_module_ref(&opt.field.position.file);
        let def_config_path = format_definition_config_path(module_ref, &opt.field.type_name);

        src.line(format!(
            "resolve: {}.{}",
            def_config_path,
            naming::field_name(&def.name)
        ));
    } else {
        src.line("resolve: nil");
    }

    src.dedent();
    src.line("),");
}

fn render_arg(src: &mut SourceCode, def: &InputValue) {
    src.line(format!("{:?}: .init(", def.name));
    src.indent();
    src.line(format!("type: {}", format_type_expression(&def.field_type)));

    if let Some(deprecation_reason) = &def.deprecation_reason {
        src.append(",");
        src.line(format!("deprecation_reason: {:?}", deprecation_reason));
    }
    if let Some(description) = &def.description {
        src.append(",");
        src.line(format!("description: {:?}", description));
    }
    if let Some(default_value) = &def.default_value {
        src.append(",");
        src.line(format!("defaultValue: {}", format_value(default_value)));
    }

    src.dedent();
    src.line("),");
}

fn render_input_field(src: &mut SourceCode, def: &InputValue) {
    src.line(format!("\"{}\": .init(", def.name));
    src.indent();

    src.line(format!("type: {}", format_type_expression(&def.field_type)));
    if let Some(description) = &def.description {
        src.append(",");
        src.line(format!("description: {:?}", description));
    }
    if let Some(deprecation_reason) = &def.deprecation_reason {
        src.append(",");
        src.line(format!("deprecationReason: {:?}", deprecation_reason));
    }

    src.dedent();
    src.line("),");
}

fn format_type_expression(t: &TypeExpression) -> String {
    match t {
        TypeExpression::NamedType(name) => match name.as_str() {
            "String" => "GraphQLString".to_string(),
            "Int" => "GraphQLInt".to_string(),
            "Float" => "GraphQLFloat".to_string(),
            "Boolean" => "GraphQLBoolean".to_string(),
            "ID" => "GraphQLID".to_string(),
            _ => format_type_reference(name),
        },
        TypeExpression::ListType(inner) => {
            format!("GraphQLList({})", format_type_expression(inner))
        }
        TypeExpression::NonNullType(inner) => {
            format!("GraphQLNonNull({})", format_type_expression(inner))
        }
    }
}

fn format_type_reference(t: &str) -> String {
    format!("GraphQLTypeReference({:?})", t)
}

fn format_value(t: &Value) -> String {
    match t {
        Value::Int(inner) => {
            format!("{}", inner)
        }
        Value::Float(inner) => {
            format!("{}", inner)
        }
        Value::String(inner) => {
            format!("{:?}", inner)
        }
        Value::Boolean(inner) => {
            if *inner {
                "true".to_owned()
            } else {
                "false".to_owned()
            }
        }
        Value::Enum(inner) => {
            format!("{:?}", inner)
        }
        Value::Null => "nil".to_owned(),
        Value::List(inner) => {
            let elements: Vec<String> = inner.iter().map(format_value).collect();
            format!("[{}]", elements.join(", "))
        }
        Value::Object(inner) => {
            let entries: Vec<String> = inner
                .iter()
                .map(|(name, value)| format!("{:?}: {}", name, format_value(value)))
                .collect();

            format!("[{}]", entries.join(", "))
        }
    }
}

fn format_definition_config_path(module: ModuleRef, name: &str) -> String {
    let breadcrumbs = module.get_breadcrumbs();
    format!("config.{}.{}", breadcrumbs.join("."), name)
}
