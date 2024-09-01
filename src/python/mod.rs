use error::Error;

use crate::schema::Project;
use std::path::PathBuf;
pub use type_expr::{format_named_source, format_type_expression};

mod builder;
mod builder_config;
mod error;
mod impl_;
mod naming;
mod source;
mod sourcecode;
mod spec;
mod type_expr;

pub fn generate_python_code(outdir: PathBuf, schema: &Project) -> Result<(), Error> {
    spec::render(&outdir, schema)?;
    source::render(&outdir, schema)?;
    builder::render(&outdir, schema)?;
    impl_::render(&outdir, schema)?;
    builder_config::render(&outdir, schema)?;

    Ok(())
}
