use serde_json;
use std::{fs::File, path::PathBuf};
mod build;
mod graphql;
mod model;
mod swift;


fn main() {
    let schema_dir = PathBuf::from("test/GraphQL");
    let out_dir = PathBuf::from("test/SchemaGeneration/Sources/SchemaGeneration");

    let result = build::build_schema(&schema_dir).unwrap();

    let file = File::create(out_dir.join("output.json")).unwrap();
    serde_json::to_writer_pretty(&file, &result.root_module).unwrap();

    graphql::render(&result, PathBuf::from("test/schema.graphql")).unwrap();
}
