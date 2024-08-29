use super::{InputValue, Position};

#[derive(Debug)]
pub struct DirectiveDefinition {
    pub name: String,
    pub description: Option<String>,
    pub arguments: Vec<InputValue>,
    pub repeatable: bool,
    pub locations: Vec<DirectiveLocation>,
    pub position: Position,
}

#[derive(Debug)]
pub enum DirectiveLocation {
    // executable
    Query,
    Mutation,
    Subscription,
    Field,
    FragmentDefinition,
    FragmentSpread,
    InlineFragment,

    // type_system
    Schema,
    Scalar,
    Object,
    FieldDefinition,
    ArgumentDefinition,
    Interface,
    Union,
    Enum,
    EnumValue,
    InputObject,
    InputFieldDefinition,
}
