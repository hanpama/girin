use crate::schema::Schema;

use super::violation::GraphQLValidationViolation;

pub fn validate_schema(s: &Schema) -> Result<(), Error> {
    // validate_lone_or_zero_schema_root(s)?;
    // validate_builtin_directives_not_overridden(s)?;
    // validate_builtin_scalars_not_overridden(s)?;
    // validate_type_definitions(s)?
    // validate_directive_definitions(s)?;
    Ok(())
}

pub struct Error {
    pub violations: Vec<GraphQLValidationViolation>,
}
