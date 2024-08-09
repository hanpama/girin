use error::PythonRenderingError;

use crate::definitions::Schema;
use std::path::PathBuf;

mod builder;
mod builder_config;
mod error;
mod impl_;
mod naming;
mod source;
mod sourcecode;

pub fn generate_python_code(outdir: PathBuf, schema: &Schema) -> Result<(), PythonRenderingError> {
    builder_config::render_builder_config(&outdir, schema)?;
    source::render_source_defintiion(&outdir, schema)?;
    builder::render_builder(&outdir, schema)?;

    Ok(())
}
