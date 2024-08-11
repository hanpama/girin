use super::{error::PythonRenderingError, naming, sourcecode::SourceCode};
use crate::schema::{
    SchemaElement, Field, InputValue, Input,
    Interface, InterfaceTypeExtension, Module, Object,
    ObjectExtension, Scalar, Schema, Submodule, TypeExpression,
};
use std::{fs::File, path::PathBuf};

pub fn render_builder(outdir: &PathBuf, s: &Schema) -> Result<(), PythonRenderingError> {
    let outfile = outdir.join("builder.py");

    let mut file = File::create(outfile)?;
    let mut src = SourceCode::new();

    src.import_third("graphql");
    src.import_first(".builder_config");

    src.line("def build_schema(config: builder_config.BuilderConfig) -> graphql.GraphQLSchema:");
    src.indent();

    for child in &s.children {
        render_module(&mut src, s, child);
    }
    for smod in &s.submodules {
        render_submodule(&mut src, s, smod);
    }

    src.line("return graphql.GraphQLSchema(");
    src.indent();

    if let Some(query) = &s.query {
        src.line(format!("query={},", naming::type_instance(query)));
    }
    if let Some(mutation) = &s.mutation {
        src.line(format!("mutation={},", naming::type_instance(mutation)));
    }
    if let Some(subscription) = &s.subscription {
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

fn render_module(src: &mut SourceCode, s: &Schema, m: &Module) {
    for child in &m.children {
        render_module(src, s, child);
    }
    for smod in &m.submodules {
        render_submodule(src, s, smod);
    }
}

fn render_submodule(src: &mut SourceCode, s: &Schema, sm: &Submodule) {
    for def in &sm.definitions {
        match def {
            SchemaElement::Object(inner) => render_object_type(src, s, inner),
            SchemaElement::Interface(inner) => render_interface_type(src, s, inner),
            SchemaElement::Input(inner) => render_input_type(src, s, inner),
            SchemaElement::Scalar(inner) => render_scalar_type(src, s, inner),
            _ => {}
        }
    }
}

fn render_object_type(src: &mut SourceCode, s: &Schema, def: &Object) {
    let name = naming::object_type_instance(def);
    src.line(format!("{} = graphql.GraphQLObjectType(", name));
    src.indent();
    src.line(format!("name=\"{}\",", def.name));

    src.line("fields=lambda: {");
    src.indent();
    for field in s.collect_object_fields(def) {
        render_field(src, s, field)
    }
    src.dedent();
    src.line("},");

    if def.interfaces.len() > 0 {
        src.line("interfaces=lambda: [");
        src.indent();
        for interface in &def.interfaces {
            src.line(format!("{},", naming::type_instance(interface)));
        }
        src.dedent();
        src.line("],");
    }

    src.dedent();
    src.line(")");
}

fn render_interface_type(src: &mut SourceCode, s: &Schema, def: &Interface) {
    let name = naming::interface_type_instance(def);
    src.line(format!("{} = graphql.GraphQLInterfaceType(", name));
    src.indent();
    src.line(format!("name=\"{}\",", def.name));

    src.line("fields=lambda: {");
    src.indent();
    for field in s.collect_interface_fields(def) {
        render_field(src, s, field)
    }
    src.dedent();
    src.line("},");

    if def.interfaces.len() > 0 {
        src.line("interfaces=lambda: [");
        src.indent();
        for interface in &def.interfaces {
            src.line(format!("{},", naming::type_instance(interface)));
        }
        src.dedent();
        src.line("],");
    }

    src.dedent();
    src.line(")");
}

fn render_input_type(src: &mut SourceCode, s: &Schema, def: &Input) {
    let name = naming::input_type_instance(def);
    src.line(format!("{} = graphql.GraphQLInputObjectType(", name));
    src.indent();
    src.line(format!("name=\"{}\",", def.name));

    src.line("fields=lambda: {");
    src.indent();
    for field in s.collect_input_fields(def) {
        render_input_field(src, s, field)
    }
    src.dedent();
    src.line("},");

    src.dedent();
    src.line(")");
}

fn render_scalar_type(src: &mut SourceCode, s: &Schema, def: &Scalar) {
    let name = naming::type_instance(&def.name);
    src.line(format!("{} = graphql.GraphQLScalarType(", name));
    src.indent();
    src.line(format!("name=\"{}\",", def.name));

    // src.line("serialize=lambda value: value,");
    // src.line("parse_value=lambda value: value,");
    // src.line("parse_literal=lambda ast: ast.value");

    src.dedent();
    src.line(")");
}

fn render_field(src: &mut SourceCode, s: &Schema, f: &Field) {
    src.line(format!("\"{}\": graphql.GraphQLField(", f.name));
    src.indent();

    src.line(format!("type_={},", format_type_expression(&f.field_type)));

    src.dedent();
    src.line("),");
}

fn render_input_field(src: &mut SourceCode, s: &Schema, f: &InputValue) {
    src.line(format!("\"{}\": graphql.GraphQLInputField(", f.name));
    src.indent();

    src.line(format!("type_={},", format_type_expression(&f.field_type)));

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
