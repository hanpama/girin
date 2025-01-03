use crate::{error::AnyError, schema::Project};
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

pub fn generate_python_code(outdir: PathBuf, schema: &Project) -> Result<(), AnyError> {
    runtime_spec::render(&outdir, schema).unwrap(); // TODO: remove unwrap
    source_spec::render(&outdir, schema).unwrap(); // TODO: remove unwrap
    builder::render(&outdir, schema).unwrap(); // TODO: remove unwrap
    runtime::render(&outdir, schema).unwrap(); // TODO: remove unwrap
    builder_config::render(&outdir, schema).unwrap(); // TODO: remove unwrap

    Ok(())
}
