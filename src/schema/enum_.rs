use super::Position;

#[derive(Debug)]
pub struct Enum {
    pub name: String,
    pub description: Option<String>,
    pub values: Vec<EnumValue>,
    pub position: Position,
}

#[derive(Debug)]
pub struct EnumValue {
    pub name: String,
    pub description: Option<String>,
    pub deprecation_reason: Option<String>,
    pub position: Position,
}


#[derive(Debug)]
pub struct EnumExtension {
    pub name: String,
    pub values: Vec<EnumValue>,
    pub position: Position,
}
