use super::{error::Error, naming, source_code::SourceCode};
use crate::{
    schema::{
        Definition, EnumDefinition, EnumValue, Field, InputDefinition, InputValue,
        InterfaceDefinition, ModuleRef, ObjectDefinition, ObjectExtension,
        Project, Resolve, ScalarDefinition, TypeExpression, UnionDefinition, Value,
    },
    swift::type_expr,
};
use std::{fs::File, path::PathBuf};

pub fn render(outdir: &PathBuf, s: &Project) -> Result<(), Error> {
    let outfile = outdir.join("Runtime.swift");

    let mut file = File::create(outfile)?;
    let mut src = SourceCode::new();

    src.import("GraphQL");

    src.line("public struct Runtime {");
    src.indent();

    src.line("public let schema: GraphQLSchema");

    src.line("public struct Wiring {");
    src.indent();
    render_directory_spec(&mut src, ModuleRef::new(s))?;
    src.dedent();
    src.line("}");

    render_init(&mut src, s);

    src.dedent();
    src.line("}");

    src.write_to(&mut file)?;

    Ok(())
}

fn render_directory_spec(src: &mut SourceCode, d: ModuleRef) -> Result<(), Error> {
    if d.has_children() {
        for child in d.iter_children() {
            let name = naming::module_name(child.get_name());
            src.line(format!("struct {name} {{"));
            src.indent();
            render_directory_spec(src, child)?;
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
                Definition::ObjectExtension(inner) => {
                    render_object_ext_spec(src, d.schema, inner);
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
                Definition::ObjectExtension(inner) => {
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
            render_field_resolver_spec(src, &resolve);
        }
    }
    src.dedent();
    src.line("}");
}

fn render_object_ext_spec(src: &mut SourceCode, s: &Project, def: &ObjectExtension) {
    let name = naming::runtime_spec(&def.name);
    src.line(&format!("struct {name} {{"));
    src.indent();

    for field in def.iter_fields() {
        if let Some(resolve) = s.resolve_field_resolve(field) {
            render_field_resolver_spec(src, &resolve);
        }
    }

    src.dedent();
    src.line("}");
}

fn render_field_resolver_spec(src: &mut SourceCode, resolve: &Resolve) {
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
        "var {name}: (_ source: {source_type}, {arguments}_ context: Any, _ info: GraphQL.GraphQLResolveInfo) {sig} -> {return_type}",
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

    format!("_ args: ({})", els.join(", "))
}

fn render_init(src: &mut SourceCode, s: &Project) {
    src.line("init(wiring: Wiring, encoder: GraphQL.MapEncoder, decoder: GraphQL.MapDecoder) {");
    src.indent();

    for def in s.collect_type_definitions_in_order() {
        match def {
            Definition::ObjectDefinition(inner) => render_object_type(src, s, inner),
            Definition::InterfaceDefinition(inner) => render_interface_type(src, s, inner),
            Definition::InputDefinition(inner) => render_input_type(src, s, inner),
            Definition::ScalarDefinition(inner) => render_scalar_type(src, s, inner),
            Definition::EnumDefinition(inner) => render_enum_type(src, s, inner),
            Definition::UnionDefinition(inner) => render_union_type(src, s, inner),
            _ => {}
        }
    }

    src.line("self.schema = try! GraphQL.GraphQLSchema(");
    src.indent();

    if let Some(query) = &s.get_query() {
        let name = naming::type_instance(query);
        src.line(format!("query: {name}"));
    }
    if let Some(mutation) = &s.get_mutation() {
        let name = naming::type_instance(mutation);
        src.append(",");
        src.line(format!("mutation: {name}"));
    }
    if let Some(subscription) = &s.get_subscription() {
        let name = naming::type_instance(subscription);
        src.append(",");
        src.line(format!("subscription: {name}"));
    }
    src.append(",");
    src.line("types: [");
    src.indent();
    for def in s.iter_type_definitions() {
        let name = naming::type_instance(def.get_definition_name().unwrap());
        src.line(format!("{name},"));
    }
    src.dedent();
    src.line("]");

    src.dedent();
    src.line(")");

    src.dedent();
    src.line("}");
}

fn render_object_type(src: &mut SourceCode, s: &Project, def: &ObjectDefinition) {
    let name = naming::type_instance(&def.name);
    src.line(format!("let {} = try! GraphQL.GraphQLObjectType(", name));
    src.indent();
    src.line(format!("name: \"{}\"", def.name));

    if let Some(description) = &def.description {
        src.append(",");
        src.line(format!("description: {:?}", description));
    }

    src.append(",");
    src.line("fields: [");
    src.indent();
    for (i, field) in s.collect_fields(&def.name).iter().enumerate() {
        if i > 0 {
            src.append(",");
        }
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
    src.line(format!("let {} = try! GraphQL.GraphQLInterfaceType(", name));
    src.indent();
    src.line(format!("name: \"{}\"", def.name));

    if let Some(description) = &def.description {
        src.append(",");
        src.line(format!("description: {:?}", description));
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
    for (i, field) in s.collect_fields(&def.name).iter().enumerate() {
        if i > 0 {
            src.append(",");
        }
        render_field(src, s, field)
    }
    src.dedent();
    src.line("]");

    src.dedent();
    src.line(")");
}

fn render_input_type(src: &mut SourceCode, s: &Project, def: &InputDefinition) {
    let name = naming::type_instance(&def.name);
    src.line(format!(
        "let {} = try! GraphQL.GraphQLInputObjectType(",
        name
    ));
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
    src.line(format!("let {} = try! GraphQL.GraphQLScalarType(", name));
    src.indent();
    src.line(format!("name: {:?},", def.name));

    if let Some(description) = &def.description {
        src.line(format!("description: {:?},", description));
    }

    src.line("serialize: { value in");
    src.indent();
    src.line("guard let value = value as? Encodable else {");
    src.indent();
    src.line(format!(
        "throw GraphQLError(message: \"{} cannot represent value: \\(value)\")",
        def.name
    ));
    src.dedent();
    src.line("}");
    src.line("return try encoder.encode(value)");

    src.dedent();
    src.line("}");

    src.dedent();
    src.line(")");
}

fn render_enum_type(src: &mut SourceCode, s: &Project, def: &EnumDefinition) {
    let name = naming::type_instance(&def.name);
    src.line(format!("let {} = try! GraphQL.GraphQLEnumType(", name));
    src.indent();
    src.line(format!("name: {:?},", def.name));

    if let Some(description) = &def.description {
        src.line(format!("description: {:?},", description));
    }

    src.line("values: [");
    src.indent();
    for (i, value) in s.collect_enum_values(&def.name).enumerate() {
        if i > 0 {
            src.append(",");
        }
        render_enum_value(src, s, value)
    }
    src.dedent();
    src.line("]");

    src.dedent();
    src.line(")");
}

fn render_enum_value(src: &mut SourceCode, s: &Project, def: &EnumValue) {
    src.line(format!("{:?}: GraphQL.GraphQLEnumValue(", def.name));
    src.indent();
    src.line(format!("value: {:?}", &def.name));
    if let Some(description) = &def.description {
        src.append(",");
        src.line(format!("description: {:?}", description));
    }
    if let Some(deprecation_reason) = &def.deprecation_reason {
        src.append(",");
        src.line(format!("deprecation_reason: {:?}", deprecation_reason));
    }
    src.dedent();
    src.line(")");
}

fn render_union_type(src: &mut SourceCode, s: &Project, def: &UnionDefinition) {
    let name = naming::type_instance(&def.name);
    src.line(format!("let {} = try! GraphQL.GraphQLUnionType(", name));
    src.indent();
    src.line(format!("name: {:?}", def.name));

    if let Some(description) = &def.description {
        src.append(",");
        src.line(format!("description: {:?}", description));
    }

    src.append(",");
    src.line("types: [");
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
    src.line(format!("{:?}: GraphQL.GraphQLField(", def.name));
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
        render_field_resolver(src, s, def, &opt);
    } else {
        src.line("resolve: nil");
    }

    src.dedent();
    src.line(")");
}

fn render_field_resolver(src: &mut SourceCode, s: &Project, def: &Field, opt: &Resolve) {
    let module_ref = s.get_module_ref(&opt.field.position.file);
    let def_config_path = format_definition_config_path(module_ref, &opt.field.type_name);

    src.line("resolve: { source, args, context, eventLoopGroup, info in");
    src.indent();

    src.line(format!(
        "let source = source as! SourceSpec.{}",
        naming::source_spec(&def.type_name)
    ));
    src.line(format!(
        "let function = {}.{}",
        def_config_path,
        naming::field_name(&opt.field.name)
    ));

    if !def.args.is_empty() {
        src.line("struct Args: Decodable {");
        src.indent();
        for arg in &def.args {
            let arg_name = naming::field_name(&arg.name);
            let arg_type = type_expr::format_type_expression(Some("SourceSpec"), &arg.field_type);
            src.line(format!("var {arg_name}: {arg_type}"));
        }

        src.dedent();
        src.line("}");
        src.line("let args: Args = try decoder.decode(Args.self, from: args)");
    }

    let mut args = Vec::new();
    for arg in &def.args {
        args.push(format!("args.{}", &arg.name));
    }
    if args.len() == 1 {
        args.push("()".to_owned());
    }
    let args = args.join(", ");

    if opt.sync {
        src.line("return eventLoopGroup.next().makeSucceededFuture(");
        src.indent();
        src.line(format!("try function(source, ({args}), context, info)"));
        src.dedent();
        src.line(")");
    } else {
        src.line("return eventLoopGroup.next().makeFutureWithTask {");
        src.indent();
        src.line(format!(
            "return try await function(source, ({args}), context, info)"
        ));
        src.dedent();
        src.line("}");
    }
    src.dedent();
    src.line("}");
}

fn render_arg(src: &mut SourceCode, def: &InputValue) {
    src.line(format!("{:?}: GraphQL.GraphQLArgument(", def.name));
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
            format!(".int({})", inner)
        }
        Value::Float(inner) => {
            format!(".double({})", inner)
        }
        Value::String(inner) => {
            format!(".string({:?})", inner)
        }
        Value::Boolean(inner) => {
            if *inner {
                ".bool(true)".to_owned()
            } else {
                ".bool(false)".to_owned()
            }
        }
        Value::Enum(inner) => {
            format!(".string({:?})", inner)
        }
        Value::Null => ".null".to_owned(),
        Value::List(inner) => {
            let elements: Vec<String> = inner.iter().map(format_value).collect();
            format!(".array([{}])", elements.join(", "))
        }
        Value::Object(inner) => {
            if inner.is_empty() {
                return ".dictionary([:])".to_owned();
            }
            let entries: Vec<String> = inner
                .iter()
                .map(|(name, value)| format!("{:?}: {}", name, format_value(value)))
                .collect();

            format!(".dictionary([{}])", entries.join(", "))
        }
    }
}

fn format_definition_config_path(module: ModuleRef, name: &str) -> String {
    let breadcrumbs = module.get_breadcrumbs();
    format!("wiring.{}.{}", breadcrumbs.join("."), name)
}
