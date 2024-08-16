use super::{error::Error, naming, sourcecode::SourceCode};
use crate::schema::{
    Definition, Field, InputDefinition, InputValue, InterfaceDefinition, Module, ObjectDefinition,
    ScalarDefinition, Schema, TypeExpression, Value,
};
use std::{fs::File, path::PathBuf};

pub fn render_builder(outdir: &PathBuf, s: &Schema) -> Result<(), Error> {
    let outfile = outdir.join("builder.py");

    let mut file = File::create(outfile)?;
    let mut src = SourceCode::new();

    src.import_third("graphql");
    src.import_first(".protocol.Config");

    src.line("def build_schema(config: Config) -> graphql.GraphQLSchema:");
    src.indent();

    for def in s.iter_definitions() {
        match def {
            Definition::ObjectDefinition(inner) => render_object_type(&mut src, s, inner),
            Definition::InterfaceDefinition(inner) => render_interface_type(&mut src, s, inner),
            Definition::InputDefinition(inner) => render_input_type(&mut src, s, inner),
            Definition::ScalarDefinition(inner) => render_scalar_type(&mut src, s, inner),
            _ => {}
        }
    }

    src.line("return graphql.GraphQLSchema(");
    src.indent();

    if let Some(query) = &s.get_query() {
        src.line(format!("query={},", naming::type_instance(query)));
    }
    if let Some(mutation) = &s.get_mutation() {
        src.line(format!("mutation={},", naming::type_instance(mutation)));
    }
    if let Some(subscription) = &s.get_subscription() {
        src.line(format!(
            "subscription={},",
            naming::type_instance(subscription)
        ));
    }
    src.line("types=[");
    src.indent();
    //
    src.dedent();
    src.line("],");

    src.dedent();
    src.line(")");

    src.dedent();

    src.write_to(&mut file)?;

    Ok(())
}

fn render_object_type(src: &mut SourceCode, s: &Schema, def: &ObjectDefinition) {
    let name = naming::object_type_instance(def);
    src.line(format!("{} = graphql.GraphQLObjectType(", name));
    src.indent();
    src.line(format!("name=\"{}\",", def.name));

    src.line("fields=lambda: {");
    src.indent();
    for field in &def.fields {
        render_field(src, s, &def.module, &def.name, field)
    }
    for ext in s.collect_extentions(&def.name) {
        for field in &ext.as_object_ext().fields {
            render_field(src, s, &def.module, &def.name, field)
        }
    }
    src.dedent();
    src.line("},");

    let interfaces = s.collect_object_interfaces(&def.name);

    if interfaces.len() > 0 {
        src.line("interfaces=lambda: [");
        src.indent();
        for interface in interfaces {
            src.line(format!("{},", naming::type_instance(interface)));
        }
        src.dedent();
        src.line("],");
    }

    src.dedent();
    src.line(")");
}

fn render_interface_type(src: &mut SourceCode, s: &Schema, def: &InterfaceDefinition) {
    let name = naming::interface_type_instance(def);
    src.line(format!("{} = graphql.GraphQLInterfaceType(", name));
    src.indent();
    src.line(format!("name=\"{}\",", def.name));

    src.line("fields=lambda: {");
    src.indent();
    for field in s.collect_interface_fields(&def.name) {
        render_field(src, s, &def.module, &def.name, field)
    }
    src.dedent();
    src.line("},");

    if def.interfaces.len() > 0 {
        src.line("interfaces=lambda: [");
        src.indent();
        for interface in s.collect_interface_interfaces(&def.name) {
            src.line(format!("{},", naming::type_instance(interface)));
        }
        src.dedent();
        src.line("],");
    }

    src.dedent();
    src.line(")");
}

fn render_input_type(src: &mut SourceCode, s: &Schema, def: &InputDefinition) {
    let name = naming::input_type_instance(def);
    src.line(format!("{} = graphql.GraphQLInputObjectType(", name));
    src.indent();
    src.line(format!("name=\"{}\",", def.name));

    src.line("fields=lambda: {");
    src.indent();
    for field in s.collect_input_fields(&def.name) {
        render_input_field(src, s, field)
    }
    src.dedent();
    src.line("},");

    src.dedent();
    src.line(")");
}

fn render_scalar_type(src: &mut SourceCode, s: &Schema, def: &ScalarDefinition) {
    let name = naming::type_instance(&def.name);
    src.line(format!("{} = graphql.GraphQLScalarType(", name));
    src.indent();
    src.line(format!("name=\"{}\",", def.name));

    let config_path = format_definition_config_path(&def.module, &def.name);
    src.line(format!("serialize={config_path}.serialize,"));
    src.line(format!("parse_value={config_path}.parse_value,"));
    src.line(format!("parse_literal={config_path}.parse_literal,"));
    //     serialize
    // parse_value
    // parse_literal
    // return self.parse_value(graphql.value_from_ast_untyped(node, variables))

    src.dedent();
    src.line(")");
}

fn render_field(
    src: &mut SourceCode,
    s: &Schema,
    parent_module: &Module,
    parent_name: &str,
    def: &Field,
) {
    src.line(format!("\"{}\": graphql.GraphQLField(", def.name));
    src.indent();

    src.line(format!(
        "type_={},",
        format_type_expression(&def.field_type)
    ));

    if !def.args.is_empty() {
        src.line("args={");
        src.indent();
        for arg in &def.args {
            render_arg(src, s, arg);
        }
        src.dedent();
        src.line("},");
    }
    if let Some(opt) = def.get_resolve_option() {
        let def_config_path = format_definition_config_path(parent_module, parent_name);

        src.line(format!(
            "resolve={}.{},",
            def_config_path,
            naming::resolver_name(&opt.name)
        ));
    }
    if let Some(deprecation_reason) = &def.deprecation_reason {
        src.line(format!("deprecation_reason={:?},", deprecation_reason));
    }
    if let Some(description) = &def.description {
        src.line(format!("description={:?},", description));
    }

    src.dedent();
    src.line("),");
}

fn render_arg(src: &mut SourceCode, s: &Schema, def: &InputValue) {
    src.line(format!("\"{}\": graphql.GraphQLArgument(", def.name));
    src.indent();
    src.line(format!(
        "type_={},",
        format_type_expression(&def.field_type)
    ));

    if let Some(default_value) = &def.default_value {
        src.line(format!("default_value={},", format_value(default_value)));
    }
    if let Some(deprecation_reason) = &def.deprecation_reason {
        src.line(format!("deprecation_reason={:?},", deprecation_reason));
    }
    if let Some(description) = &def.description {
        src.line(format!("description={:?},", description));
    }

    src.dedent();
    src.line("),");
}

fn render_input_field(src: &mut SourceCode, s: &Schema, def: &InputValue) {
    src.line(format!("\"{}\": graphql.GraphQLInputField(", def.name));
    src.indent();

    src.line(format!(
        "type_={},",
        format_type_expression(&def.field_type)
    ));
    if let Some(deprecation_reason) = &def.deprecation_reason {
        src.line(format!("deprecation_reason={:?},", deprecation_reason));
    }
    if let Some(description) = &def.description {
        src.line(format!("description={:?},", description));
    }

    src.dedent();
    src.line("),");
}

fn format_type_expression(t: &TypeExpression) -> String {
    match t {
        TypeExpression::NamedType(name) => match name.as_str() {
            "String" => "graphql.GraphQLString".to_string(),
            "Int" => "graphql.GraphQLInt".to_string(),
            "Float" => "graphql.GraphQLFloat".to_string(),
            "Boolean" => "graphql.GraphQLBoolean".to_string(),
            "ID" => "graphql.GraphQLID".to_string(),
            _ => naming::type_instance(name),
        },
        TypeExpression::ListType(inner) => {
            format!("graphql.GraphQLList({})", format_type_expression(inner))
        }
        TypeExpression::NonNullType(inner) => {
            format!("graphql.GraphQLNonNull({})", format_type_expression(inner))
        }
    }
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
                "True".to_owned()
            } else {
                "False".to_owned()
            }
        }
        Value::Enum(inner) => {
            format!("{:?}", inner)
        }
        Value::Null => "None".to_owned(),
        Value::List(inner) => {
            let elements: Vec<String> = inner.iter().map(format_value).collect();
            format!("[{}]", elements.join(", "))
        }
        Value::Object(inner) => {
            let entries: Vec<String> = inner
                .iter()
                .map(|(name, value)| format!("{:?}: {}", name, format_value(value)))
                .collect();

            format!("{{{}}}", entries.join(", "))
        }
    }
}

fn format_definition_config_path(module: &Module, name: &str) -> String {
    format!("config.{}.{}", module.join("."), name)
}
