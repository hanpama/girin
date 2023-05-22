pub mod model;
mod build;

// use graphql_parser::query::{parse_query, ParseError};
use graphql_parser::schema::{parse_schema, Definition};

fn main() {
    let schema = "
      schema {
        query: Query
      }

      type Query {
        version: String
      }
    ";
    let res = parse_schema::<&str>(schema).unwrap();

    for v in res.definitions {
        match v {
            Definition::SchemaDefinition(def) => {
                // def.query
            }
            _ => {}
        }
    }

    let item = res.definitions.get(0).unwrap();

    println!("Hello, world!");
}
