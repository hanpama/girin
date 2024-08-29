use super::ScalarDefinition;
use crate::schema::Schema;
use graphql_parser::parse_schema;
use std::fs::{read_dir, DirEntry};
use std::path::Path;
use std::{fs::File, io::Read};
use validate::validate_schema;

mod construct;
mod validate;
mod violation;

pub fn load(root: &Path) -> Result<Schema, Error> {
    let mut violations = vec![];

    let mut schema = Schema::new(root.to_path_buf());
    if let Err(error) = load_directory(&mut schema, &root, &root) {
        match error {
            Error::Rule(vs) => {
                violations.extend(vs);
            }
            _ => {
                return Err(error);
            }
        }
    }
    if let Err(error) = validate_schema(&schema) {
        violations.extend(error.violations);
    }

    if !violations.is_empty() {
        return Err(Error::Rule(violations));
    }

    Ok(schema)
}

fn load_directory(s: &mut Schema, root: &Path, dir: &Path) -> Result<(), Error> {
    let mut parse_errors = vec![];
    let mut violations = vec![];

    for item in read_dir(&dir)? {
        if let Err(error) = load_dir_entry(s, root, item?) {
            match error {
                Error::Io(err) => return Err(Error::Io(err)),
                Error::Parser(errors) => {
                    parse_errors.extend(errors);
                }
                Error::Rule(vs) => {
                    violations.extend(vs);
                }
            }
        }
    }

    if !parse_errors.is_empty() {
        return Err(Error::Parser(parse_errors));
    }
    if !violations.is_empty() {
        return Err(Error::Rule(violations));
    }
    Ok(())
}

fn load_dir_entry(s: &mut Schema, root: &Path, entry: DirEntry) -> Result<(), Error> {
    if entry.file_type()?.is_dir() {
        load_directory(s, root, &entry.path())
    } else {
        load_file(s, &entry.path())
    }
}

fn load_file(s: &mut Schema, file: &Path) -> Result<(), Error> {
    let mut buf = String::new();
    File::open(file)?.read_to_string(&mut buf)?;

    let document_result = parse_schema::<String>(&buf);
    if let Err(error) = document_result {
        return Err(Error::Parser(vec![error]));
    }

    let document = document_result.unwrap();
    let definitions_result = construct::construct_definitions(file, document);

    if let Err(error) = definitions_result {
        return Err(Error::Rule(error.violations));
    }

    let definitions = definitions_result.unwrap();
    for def in definitions {
        s.add_definition(def);
    }

    Ok(())
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Parser(Vec<graphql_parser::schema::ParseError>),
    Rule(Vec<violation::GraphQLValidationViolation>),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}
