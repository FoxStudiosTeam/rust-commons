use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, Hash)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    #[serde(rename = "isPrimary")]
    #[serde(default)]
    pub is_primary: bool,
}

impl Field {
    pub fn map_types(&mut self, types_map: &HashMap<String, String>) {
        if let Some(t) = types_map.get(&self.field_type) {
            self.field_type = t.clone();
        }
    }
}

