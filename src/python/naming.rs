use crate::schema::{
    EnumDefinition, InputDefinition, InterfaceDefinition, InterfaceExtension, ObjectDefinition,
    ObjectExtension, ScalarDefinition,
};

pub fn spec_type(def_name: &str) -> String {
    format!("{}Spec", def_name)
}
pub fn impl_type(def_name: &str) -> String {
    format!("{}Impl", def_name)
}

pub fn object_source(def: &ObjectDefinition) -> String {
    format!("{}Source", def.name)
}

pub fn interface_source(def: &InterfaceDefinition) -> String {
    format!("{}Source", def.name)
}

pub fn input_source(def: &InputDefinition) -> String {
    format!("{}Source", def.name)
}

pub fn scalar_source(def: &ScalarDefinition) -> String {
    format!("{}Source", def.name)
}

pub fn enum_source(def: &EnumDefinition) -> String {
    format!("{}Source", def.name)
}

pub fn source<S: Into<String>>(def_name: S) -> String {
    format!("{}Source", def_name.into())
}

pub fn object_type_instance(def: &ObjectDefinition) -> String {
    format!("{}", def.name)
}
pub fn input_type_instance(def: &InputDefinition) -> String {
    format!("{}", def.name)
}
pub fn type_instance(def_name: &str) -> String {
    format!("{}", def_name)
}
pub fn interface_type_instance(def: &InterfaceDefinition) -> String {
    format!("{}", def.name)
}

pub fn field_name(in_schema_name: &str) -> String {
    to_snake_case(in_schema_name)
}

fn to_snake_case(s: &str) -> String {
    let mut snake_case = String::new();
    let mut i = 0;

    while i < s.len() {
        let mut matched_acronym = false;

        // Check for known acronyms
        for &acronym in &ACRONYMS {
            if s[i..].starts_with(acronym) {
                if !snake_case.is_empty() {
                    snake_case.push('_');
                }
                snake_case.push_str(&acronym.to_ascii_lowercase());
                i += acronym.len();
                matched_acronym = true;
                break;
            }
        }

        if !matched_acronym {
            let c = s.chars().nth(i).unwrap();
            if c.is_uppercase() {
                if i != 0 {
                    snake_case.push('_');
                }
                snake_case.push(c.to_ascii_lowercase());
            } else {
                snake_case.push(c);
            }
            i += 1;
        }
    }

    snake_case
}

const ACRONYMS: [&str; 42] = [
    "ACL", "API", "ASCII", "CPU", "CSS", "DNS", "EOF", "ETA", "GPU", "GUID", "HTML", "HTTP",
    "HTTPS", "ID", "IP", "JSON", "LHS", "OS", "QPS", "RAM", "RHS", "RPC", "SLA", "SMTP", "SQL",
    "SSH", "TCP", "TLS", "TTL", "UDP", "UI", "UID", "UUID", "URI", "URL", "UTF8", "VM", "XML",
    "XMPP", "XSRF", "XSS", "OAuth",
];
