use serde::{Deserialize, Serialize};
use hashbrown::HashMap;
use utils::wrappers;
use anyhow::Result;

use crate::prelude::{ErrOr, Field, Type, TypeMapping, TypedField};

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
    pub fn complete(self, types: &HashMap<String, Type>) -> Result<Table, String> {
        let Some(schema) = self.schema else {
            return Err("Schema is required".to_string());
        };
        let mut pks = vec![];
        let mut fields = vec![];
        for field in self.fields.into_iter() {
            if field.is_primary {
                pks.push(field.name.clone());
            }
            let err_msg = format!("Unknown type {}", field.type_name);
            let Some(typed_field) = field.into_typed(&crate::prelude::TypeMapping::default(), &types) else {
                return Err(err_msg);
            };
            fields.push(typed_field);
        }
        if pks.len() > 1 {
            return Err(format!("Table {} has multiple primary keys: {}", self.name, pks.join(", ")));
        }
        if pks.len() == 0 && !self.is_abstract {
            return Err(format!("Table {} has no primary key", self.name));
        }
        Ok(Table {
            name: self.name,
            schema,
            fields,
        })
    }
}

#[derive(Clone, Deserialize, Serialize, Default, Debug, PartialEq, Hash)]
pub struct Table {
    pub name: String,
    pub schema: String,
    pub fields: Vec<TypedField>,
}

#[derive(Debug, Serialize)]
pub struct TableChanged {
    pub name: String,
    pub schema: String,
    pub added: Vec<TypedField>,
    pub removed: Vec<TypedField>, 
    pub changed: Vec<ChangedField>,
}

impl TableChanged {
    pub fn map_types(&mut self, target: &TypeMapping, types_map: &HashMap<String, Type>) -> Result<()> {
        for field in self.changed.iter_mut() {
            field.map_type(target, types_map)?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct ChangedField {
    pub name: String,
    pub type_name: String,
    pub type_str: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pk: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unique: Option<bool>,
}

impl ChangedField {
    fn from_difference(kept: &TypedField, other: &TypedField) -> Option<Self> {
        if kept == other {
            return None;
        }

        if other.type_name != kept.type_name || other.type_str != kept.type_str {
            tracing::warn!(
                "Type of field {} changed from {}({}) to {}({}). You will MUST manually complete the SQL migration.", 
                kept.name, kept.type_name, kept.type_str, other.type_name, other.type_str
            );
        }
        let mut changed = false;
        let pk = if other.is_primary != kept.is_primary { changed = true; Some(other.is_primary) } else { None };
        let default = if other.default != kept.default { changed = true; other.default.clone() } else { None };
        let nullable = if other.nullable != kept.nullable { changed = true; Some(other.nullable) } else { None };
        let is_unique = if other.is_unique != kept.is_unique { changed = true; Some(other.is_unique) } else { None };

        if !changed {
            None
        } else {
            Some(Self {
                name: other.name.clone(),
                type_name: other.type_name.clone(),
                type_str: other.type_str.clone(),
                pk,
                default,
                nullable,
                is_unique,
            })
        }
    }

    pub fn map_type(&mut self, target: &TypeMapping, mappings: &HashMap<String, Type>) -> anyhow::Result<()> {
        self.type_str = mappings.get(&self.type_name).or_err::<anyhow::Error>("Unknown type")?.get_mapping(&target).to_string();
        Ok(())
    }
}

wrappers!(
    #[derive(Debug, Serialize)]
    pub TableAdded(pub Table)

    #[derive(Debug, Serialize)]
    pub TableRemoved(pub Table)
);

impl Table {
    pub fn map_types(&mut self, target: &TypeMapping, types_map: &HashMap<String, Type>) -> Result<()> {
        for field in self.fields.iter_mut() {
            field.map_type(target, types_map)?;
        }
        Ok(())
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