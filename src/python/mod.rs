use error::Error;

use crate::schema::Project;
use std::path::PathBuf;
pub use type_expr::format_type_expression;

mod builder;
mod builder_config;
mod error;
mod naming;
mod runtime;
mod runtime_spec;
mod source_code;
mod source_spec;
mod type_expr;

pub fn generate_python_code(outdir: PathBuf, schema: &Project) -> Result<(), Error> {
    runtime_spec::render(&outdir, schema)?;
    source_spec::render(&outdir, schema)?;
    builder::render(&outdir, schema)?;
    runtime::render(&outdir, schema)?;
    builder_config::render(&outdir, schema)?;

    Ok(())
}
