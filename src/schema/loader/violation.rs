use crate::schema::Position;

#[derive(Debug)]
pub enum GraphQLValidationViolation {
    ProvidedRequiredArguments {
        pos: Position,
        name: String,
    },
    KnownArgumentNames {
        pos: Position,
        name: String,
    },
    ValuesOfCorrectType {
        pos: Position,
        name: String,
        value: String,
    },
    LoneSchemaDefinition {
        pos: Position,
    },
    PossibleTypeExtensions {
        pos: Position,
        def: String,
        ext: String,
    },
    UniqueArgumentDefinitionNames {
        pos: Position,
        name: String,
    },
    UniqueDirectiveNames {
        pos: Position,
        name: String,
    },
    UniqueEnumValueNames {
        pos: Position,
        name: String,
    },
    UniqueFieldDefinitionNames {
        pos: Position,
        name: String,
    },
    UniqueTypeNames {
        pos: Position,
        name: String,
    },
}

impl std::fmt::Display for GraphQLValidationViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GraphQLValidationViolation::ProvidedRequiredArguments { pos, name } => {
                write!(f, "{pos} - Should provide required argument: {name}")
            }
            GraphQLValidationViolation::KnownArgumentNames { pos, name } => {
                write!(f, "{pos} - Unknown argument name: {name}")
            }
            GraphQLValidationViolation::ValuesOfCorrectType { pos, name, value } => {
                write!(f, "{pos} - Invalid value type: {name} {value}")
            }
            GraphQLValidationViolation::LoneSchemaDefinition { pos } => {
                write!(f, "{pos} - Lone schema definition violation")
            }
            GraphQLValidationViolation::PossibleTypeExtensions { pos, def, ext } => {
                write!(f, "Possible type extensions violation")
            }
            GraphQLValidationViolation::UniqueArgumentDefinitionNames { pos, name } => {
                write!(f, "Unique argument definition names violation: {}", name)
            }
            GraphQLValidationViolation::UniqueDirectiveNames { pos, name } => {
                write!(f, "Unique directive names violation: {}", name)
            }
            GraphQLValidationViolation::UniqueEnumValueNames { pos, name } => {
                write!(f, "Unique enum value names violation: {}", name)
            }
            GraphQLValidationViolation::UniqueFieldDefinitionNames { pos, name } => {
                write!(f, "Unique field definition names violation: {}", name)
            }
            GraphQLValidationViolation::UniqueTypeNames { pos, name } => {
                write!(f, "Unique type names violation: {}", name)
            }
        }
    }
}
