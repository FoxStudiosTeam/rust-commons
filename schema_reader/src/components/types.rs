use serde::{Deserialize, Serialize};

use crate::prelude::TypeMapping;


#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
pub struct Type {
    #[serde(rename = "rustType")]
    pub rust_type: String,
    #[serde(rename = "pgType")]
    pub pg_type: String,
}

impl Type {
    pub fn get_mapping(&self, target: &TypeMapping) -> &str {
        match target {
            TypeMapping::Rust => &self.rust_type,
            TypeMapping::Pg => &self.pg_type,
        }
    }
}