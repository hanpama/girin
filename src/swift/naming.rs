pub fn resolver_spec(def_name: &str) -> String {
    format!("{}", def_name)
}

pub fn source_spec<S: Into<String>>(def_name: S) -> String {
    format!("{}", def_name.into())
}

pub fn field_name<S: Into<String>>(field_name: S) -> String {
    format!("{}", field_name.into())
}

pub fn type_instance(def_name: &str) -> String {
    format!("{}Definition", def_name)
}
pub fn type_defining_function(def_name: &str) -> String {
    format!("define{}", def_name)
}

// TODO: "GraphQL"은 escape해야 함
