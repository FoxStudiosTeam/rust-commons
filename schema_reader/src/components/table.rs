use serde::{Deserialize, Serialize};
use hashbrown::HashMap;
use utils::wrappers;

use crate::prelude::{Field, Type};

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct RawTable {
    pub name: String,
    #[serde(rename="abstract")]
    #[serde(default)]
    pub is_abstract: bool,
    pub extends: Option<String>,
    pub schema: Option<String>,
    pub fields: Vec<Field>,
}

impl RawTable {
    pub fn extend(&mut self, other: RawTable) {
        self.fields.extend(other.fields);
        if self.schema.is_none() {
            self.schema = other.schema;
        }
    }
}


impl RawTable {
    pub fn complete(mut self, types: &HashMap<String, Type>) -> Result<Table, String> {
        let Some(schema) = self.schema else {
            return Err("Schema is required".to_string());
        };
        for field in self.fields.iter_mut() {
            let Some(t) = types.get(&field.field_type) else {
                return Err(format!("Unknown type {}", field.field_type));
            };
            field.field_type = t.inner.clone();
        }
        Ok(Table {
            name: self.name,
            schema,
            fields: self.fields,
        })
    }
}

#[derive(Clone, Deserialize, Serialize, Default, Debug, PartialEq, Hash)]
pub struct Table {
    pub name: String,
    pub schema: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Serialize)]
pub struct TableChanged {
    pub name: String,
    pub schema: String,
    pub added: Vec<Field>,
    pub removed: Vec<Field>, 
    pub changed: Vec<ChangedField>,
}

impl TableChanged {
    pub fn map_types(&mut self, types_map: &HashMap<String, String>) {
        for field in self.added.iter_mut() {
            field.map_types(types_map);
        }
        for field in self.removed.iter_mut() {
            field.map_types(types_map);
        }
        for field in self.changed.iter_mut() {
            field.map_types(types_map);
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ChangedField {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pk: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
}

impl ChangedField {
    fn from_difference(kept: &Field, other: &Field) -> Option<Self> {
        if kept == other {
            return None;
        }
        let pk = if other.is_primary != kept.is_primary { Some(other.is_primary) } else { None };
        let field_type = if other.field_type != kept.field_type {Some(other.field_type.clone())} else { None };
        if pk.is_none() && field_type.is_none() {
            None
        } else {
            Some(Self {name: kept.name.clone(), pk, field_type})
        }
    }
    fn map_types(&mut self, types_map: &HashMap<String, String>) {
        let Some(k) = &self.field_type else {return;};
        if let Some(t) = types_map.get(k) {
            self.field_type = Some(t.clone());
        }
    }
}

wrappers!(
    #[derive(Debug, Serialize)]
    pub TableAdded(pub Table)

    #[derive(Debug, Serialize)]
    pub TableRemoved(pub Table)
);

impl Table {
    pub fn map_types(&mut self, types_map: &HashMap<String, String>) {
        for field in self.fields.iter_mut() {
            if let Some(t) = types_map.get(&field.field_type) {
                field.field_type = t.clone();
            }
        }
    } 
    pub fn difference(&self, other: &Table) -> Option<TableChanged> {
        if self.schema != other.schema || self.name != other.name {
            return None;
        }
        let mut added = vec![];
        let mut removed = self.fields.clone()
            .into_iter()
            .map(|v| (v.name.clone(), v))
            .collect::<HashMap<_, _>>();
        let mut changed = vec![];

        for field in other.fields.iter() {
            if let Some(kept) = removed.remove(&field.name) {
                ChangedField::from_difference(&kept, field)
                    .map(|v| changed.push(v));
            } else {
                added.push(field.clone());
            }
        }
        changed.sort_by(|a, b| a.pk.cmp(&b.pk));

        Some(TableChanged {
            name: self.name.clone(),
            schema: self.schema.clone(),
            added,
            removed: removed
                .into_iter()
                .map(|(_k, v)|v)
                .collect(),
            changed
        })
    }
}