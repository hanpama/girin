use std::{fs::File, io::Write, path::PathBuf};
mod build;
mod graphql;
mod python;
mod schema;
mod swift;

fn main() {
    let schema_dir = PathBuf::from("test/GraphQL");
    let out_dir = PathBuf::from("test/SchemaGeneration/Sources/SchemaGeneration");

    let result = build::build_schema(&schema_dir).unwrap();

    let mut result_debug_file = File::create(out_dir.join("debug")).unwrap();
    let result_debug = format!("{:#?}", result);
    result_debug_file
        .write_all(result_debug.as_bytes())
        .unwrap();

    graphql::render(&result, PathBuf::from("test/schema.graphql")).unwrap();
    python::generate_python_code(PathBuf::from("test/python"), &result).unwrap();
}
