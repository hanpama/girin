use clap::Command;
use std::path::PathBuf;
mod error;
mod graphql;
mod python;
mod schema;
mod swift;
mod utils;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let cli = Command::new("girin")
        .bin_name("girin")
        .version(VERSION)
        .author("Kyungil Choi <hanpama@gmail.com>")
        .about("GraphQL code generator")
        .subcommand_required(true)
        .subcommand(
            Command::new("swift")
                .about("Generate Swift code")
                .arg(clap::arg!(-s --schema <Directory>))
                .arg(clap::arg!(-o --out <Directory>)),
        )
        .subcommand(
            Command::new("python")
                .about("Generate Python code")
                .arg(clap::arg!(-s --schema <Directory>))
                .arg(clap::arg!(-o --out <Directory>)),
        )
        .subcommand(
            Command::new("graphql")
                .about("Compile to a single GraphQL schema file")
                .arg(clap::arg!(-s --schema <Directory>))
                .arg(clap::arg!(-o --out <File>)),
        );

    let matches = cli.get_matches();

    let res = match matches.subcommand() {
        Some(("swift", args)) => {
            let schema_dir = args.get_one::<String>("schema").unwrap();
            let outdir = args.get_one::<String>("out").unwrap();
            let schema_dir = PathBuf::from(schema_dir);
            let out_dir = PathBuf::from(outdir);
            let prj = schema::load(&schema_dir).unwrap();
            swift::generate_swift_code(out_dir, &prj)
        }
        Some(("python", args)) => {
            let schema_dir = args.get_one::<String>("schema").unwrap();
            let outfile = args.get_one::<String>("out").unwrap();
            let schema_dir = PathBuf::from(schema_dir);
            let outfile = PathBuf::from(outfile);
            let prj = schema::load(&schema_dir).unwrap();

            python::generate_python_code(outfile, &prj)
        }
        Some(("graphql", args)) => {
            let schema_dir = args.get_one::<String>("schema").unwrap();
            let outfile = args.get_one::<String>("out").unwrap();
            let schema_dir = PathBuf::from(schema_dir);
            let outfile = PathBuf::from(outfile);
            let prj = schema::load(&schema_dir).unwrap();
            graphql::render(&prj, outfile)
        }
        _ => unreachable!(),
    };
    if let Err(e) = res {
        eprintln!("{}", e);
    }
}
