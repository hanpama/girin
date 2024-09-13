use crate::schema::Project;
use std::{error::Error, path::PathBuf};
mod builder;
mod error;
mod naming;
mod type_expr;
mod source_spec;
mod source_code;
mod runtime;
mod runtime_spec;

pub fn generate_swift_code(outdir: PathBuf, schema: &Project) -> Result<(), SwiftGenerationError> {
    source_spec::render(&outdir, schema).unwrap();
    builder::render(&outdir, schema).unwrap();
    runtime_spec::render(&outdir, schema).unwrap();
    runtime::render(&outdir, schema).unwrap();

    Ok(())
}

#[derive(Debug)]
pub enum SwiftGenerationError {
    IoError(std::io::Error),
}

impl Error for SwiftGenerationError {}

impl From<std::io::Error> for SwiftGenerationError {
    fn from(e: std::io::Error) -> Self {
        SwiftGenerationError::IoError(e)
    }
}

impl std::fmt::Display for SwiftGenerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SwiftGenerationError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}
