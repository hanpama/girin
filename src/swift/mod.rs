use std::{error::Error, path::PathBuf};

mod sourcecode;

pub fn generate_swift_code(target_dir: PathBuf) -> Result<(), SwiftGenerationError> {
    // let source = source::Source::new(module);
    // source.generate()

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
