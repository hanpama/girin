use crate::schema::{
    Enum, Input, Interface, InterfaceTypeExtension, Object, ObjectExtension, Scalar,
    TypeDefinition, TypeExtension,
};

// pub fn module_config_type(md: &Module) -> String {
//     format!("{}", md.name)
// }

// pub fn module_config_field_name(md: &Module) -> String {
//     format!("_{}", md.name)
// }

// pub fn submodule_config_type(smd: &Submodule) -> String {
//     format!("{}", smd.name)
// }

// pub fn submodule_config_field_name(smd: &Submodule) -> String {
//     format!("_{}", smd.name)
// }

pub fn def_object_resolver_type(def: &Object) -> String {
    format!("{}Config", def.name)
}
pub fn def_interface_resolver_type(def: &Interface) -> String {
    format!("{}Config", def.name)
}
pub fn ext_object_resolver_type(def: &ObjectExtension) -> String {
    format!("{}Config", def.name)
}
pub fn ext_interface_resolver_type(def: &InterfaceTypeExtension) -> String {
    format!("{}Config", def.name)
}

pub fn object_source(def: &Object) -> String {
    format!("{}Source", def.name)
}

pub fn interface_source(def: &Interface) -> String {
    format!("{}Source", def.name)
}

pub fn input_source(def: &Input) -> String {
    format!("{}Source", def.name)
}

pub fn scalar_source(def: &Scalar) -> String {
    format!("{}Source", def.name)
}

pub fn enum_source(def: &Enum) -> String {
    format!("{}Source", def.name)
}

pub fn source<S: Into<String>>(def_name: S) -> String {
    format!("{}Source", def_name.into())
}

pub fn object_type_instance(def: &Object) -> String {
    format!("{}", def.name)
}
pub fn input_type_instance(def: &Input) -> String {
    format!("{}", def.name)
}
pub fn type_instance(def_name: &str) -> String {
    format!("{}", def_name)
}
pub fn interface_type_instance(def: &Interface) -> String {
    format!("{}", def.name)
}

pub fn resolver_name(in_schema_name: &str) -> String {
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
