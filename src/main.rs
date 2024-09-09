use std::{fs::File, io::Write, path::PathBuf};

use clap::Command;
mod graphql;
mod python;
mod schema;
mod swift;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // let cli = Command::new("girin")
    //     .bin_name("girin")
    //     .version(VERSION)
    //     .author("Kyungil Choi <hanpama@gmail.com>")
    //     .about("GraphQL code generator")
    //     .subcommand_required(true)
    //     .subcommand(Command::new("python").about("Generate Python code"));

    let schema_dir = PathBuf::from("test/GraphQL");
    // let out_dir = PathBuf::from("test/SchemaGeneration/Sources/SchemaGeneration");

    let result = schema::load(&schema_dir).unwrap();

    // let mut result_debug_file = File::create(out_dir.join("debug")).unwrap();
    // let result_debug = format!("{:#?}", result);
    // result_debug_file
    //     .write_all(result_debug.as_bytes())
    //     .unwrap();

    graphql::render(&result, PathBuf::from("test/schema.graphql")).unwrap();
    python::generate_python_code(PathBuf::from("test/python/myproject"), &result).unwrap();
}
