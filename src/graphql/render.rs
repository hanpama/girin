use std::error::Error;
use std::{fs::File, io::Write, path::PathBuf};

use crate::model::Schema;

use super::ast;

pub fn render(s: &Schema, outfile: PathBuf) -> Result<(), GraphQLRenderingError> {
    let schema_ast = ast::build_schema_ast(s);

    let mut file = File::create(outfile)?;
    file.write_all(&schema_ast.to_string().as_bytes())?;

    return Ok(());
}

#[derive(Debug)]
pub enum GraphQLRenderingError {
    IoError(std::io::Error),
}

impl Error for GraphQLRenderingError {}

impl From<std::io::Error> for GraphQLRenderingError {
    fn from(e: std::io::Error) -> Self {
        GraphQLRenderingError::IoError(e)
    }
}

impl std::fmt::Display for GraphQLRenderingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GraphQLRenderingError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}
