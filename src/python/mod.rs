use error::Error;

use crate::schema::Schema;
use std::path::PathBuf;

mod builder;
mod protocol;
mod error;
mod impl_;
mod naming;
mod source;
mod sourcecode;

pub fn generate_python_code(outdir: PathBuf, schema: &Schema) -> Result<(), Error> {
    protocol::render_builder_config(&outdir, schema)?;
    source::render_source_defintiion(&outdir, schema)?;
    builder::render_builder(&outdir, schema)?;

    Ok(())
}
