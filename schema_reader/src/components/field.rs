use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use crate::prelude::{ErrOr, Type, TypeMapping};

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, Hash)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub type_name: String,
    #[serde(rename = "isPrimary")]
    #[serde(default)]
    pub is_primary: bool,
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(rename = "nullable", skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    #[serde(rename = "isUnique")]
    pub is_unique: Option<bool>
}

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, Hash)]
pub struct TypedField {
    pub name: String,
    pub type_name: String,
    pub type_str: String,
    pub is_primary: bool,
    pub default: Option<String>,
    pub nullable: bool,
    pub is_unique: bool,
    #[cfg(feature = "utoipa_gen")]
    pub utoipa_example: String,
}

impl Field {
    pub fn into_typed(self, target: &TypeMapping, mappings: &HashMap<String, Type>) -> Option<TypedField> {
        let field_type = mappings.get(&self.type_name)?;
        let type_str = field_type.get_mapping(target).to_string();
        Some(
            TypedField {
                name: self.name, 
                type_name: self.type_name, 
                type_str, 
                is_primary: self.is_primary,
                is_unique: self.is_unique.unwrap_or(false),
                default: self.default.clone(),
                nullable: self.nullable.unwrap_or(false),
                #[cfg(feature = "utoipa_gen")]
                utoipa_example: field_type.utoipa_example.clone(),
            }
        )
    }
}

impl TypedField {
    pub fn map_type(&mut self, target: &TypeMapping, mappings: &HashMap<String, Type>) -> anyhow::Result<()> {
        self.type_str = mappings.get(&self.type_name).or_err::<anyhow::Error>("Unknown type")?.get_mapping(&target).to_string();
        Ok(())
    }
}


