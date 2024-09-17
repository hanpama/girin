use super::ScalarDefinition;
use crate::schema::Project;
use std::fs::read_dir;
use std::path::Path;
use std::{fs::File, io::Read};
use validate::validate_schema;

mod construct;
mod type_expr;
mod validate;
mod violation;

pub fn load(root: &Path) -> Result<Project, Error> {
    let mut violations = vec![];

    let mut prj = Project::new(root.to_path_buf());
    if let Err(error) = load_directory(&mut prj, &root, &root) {
        match error {
            Error::Rule(vs) => {
                violations.extend(vs);
            }
            _ => {
                return Err(error);
            }
        }
    }
    if let Err(error) = validate_schema(&prj) {
        violations.extend(error.violations);
    }

    if !violations.is_empty() {
        return Err(Error::Rule(violations));
    }

    Ok(prj)
}

fn load_directory(prj: &mut Project, root: &Path, dir: &Path) -> Result<(), Error> {
    let mut parse_errors = vec![];
    let mut violations = vec![];

    for entry in read_dir(&dir)? {
        let entry = entry?;
        let dir_entry_result = if entry.file_type()?.is_dir() {
            load_directory(prj, root, &entry.path())
        } else {
            load_file(prj, &entry.path())
        };
        if let Err(error) = dir_entry_result {
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

fn load_file(prj: &mut Project, file: &Path) -> Result<(), Error> {
    let mut buf = String::new();
    File::open(file)?.read_to_string(&mut buf)?;

    let document = graphql_parser::parse_schema::<String>(&buf)?;
    let definitions = construct::construct_definitions(file, document)?;
    for def in definitions {
        prj.add_definition(def);
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

impl From<graphql_parser::schema::ParseError> for Error {
    fn from(err: graphql_parser::schema::ParseError) -> Self {
        Error::Parser(vec![err])
    }
}

impl From<construct::Error> for Error {
    fn from(err: construct::Error) -> Self {
        Error::Rule(err.violations)
    }
}
