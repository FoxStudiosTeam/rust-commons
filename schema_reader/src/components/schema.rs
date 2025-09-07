use std::{fs, path::Path};

use serde::{Deserialize, Serialize};
use hashbrown::HashMap;
use anyhow::Result;
use tracing::{error, warn};

use crate::prelude::{RawTable, RenderScheme, Table, Type};

#[derive(Clone, Deserialize, Default, Debug)]
struct RawYamlSchema {
    pub tables: Vec<RawTable>,
    #[serde(default)]
    pub types: HashMap<String, Type>,
}

impl RawYamlSchema {
    fn flatten(self) -> Result<Schema> {
        let tables: HashMap<String, RawTable> = self.tables.into_iter().map(|t| (t.name.clone(), t)).collect();
        let mut flatten_tables: HashMap<String, Table> = Default::default();

        let mut abstract_tables : HashMap<String, RawTable> = Default::default();
        // Complete abstract tables
        for (table_name, table) in tables.iter() {
            if !table.is_abstract {continue;};

            let mut table = table.clone();
            let mut extends = table.extends.clone(); 
            // Flatten abstract table hierarchy
            while let Some(parent) = &extends {
                let Some(parent) = tables.get(parent) else {
                    error!("Table {} extends unknown table {}", table_name, parent);
                    continue;
                };
                extends = parent.extends.clone();
                table.extend(parent.clone());
            }
            abstract_tables.insert(table_name.clone(), table);
        }

        // Complete non-abstract tables
        for (table_name, table) in tables.iter() {
            if table.is_abstract {continue;};

            let mut table = table.clone();
            if let Some(extends) = &table.extends {
                let Some(parent) = tables.get(extends) else {
                    error!("Table {} extends unknown table {}", table_name, extends);
                    continue;
                };
                table.extend(parent.clone());
            }
            flatten_tables.insert(table_name.clone(), table.complete(&self.types).map_err(|e: String| anyhow::anyhow!(e))?);
        }
        Ok(Schema { tables: flatten_tables, types: self.types, type_mapping: TypeMapping::Rust })
    }

    fn from_dir<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut result = Self::default();
        for entry in fs::read_dir(&path)? {
            let e: Result<()> = (||{
                let entry = entry?.path();
                if entry.is_file() {
                    let content = Self::from_file(entry)?;
                    result.extend(content);
                }
                Ok(())
            })(); 
            if let Err(e) = e {
                warn!("Failed to read schema file, skipping: {}", e);
            }
        }
        Ok(result)
    }
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())?;
        Ok(serde_yaml::from_str(&content)?)
    }
    fn extend(&mut self, schema: Self) {
        self.tables.extend(schema.tables);
        self.types.extend(schema.types);
    }
}

#[derive(Clone, Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct Schema {
    pub tables: HashMap<String, Table>,
    pub types: HashMap<String, Type>,
    pub type_mapping: TypeMapping,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug, PartialEq, Eq)]
pub enum TypeMapping {
    #[default]
    Rust,
    Pg,
}


impl Schema {
    pub fn from_dir<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(RawYamlSchema::from_dir(path)?.flatten()?)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(RawYamlSchema::from_file(path)?.flatten()?)
    }

    pub fn extend(&mut self, schema: Self) {
        for (k, v) in self.tables.iter() {
            let Some(t) = schema.tables.get(k) else {continue;};
            if !t.eq(v) {
                warn!("Overlapping table defined in multiple files. Will be overwritten: {}: {} overrides {}", k, t.name, v.name);
            }
        }
        self.tables.extend(schema.tables);

        for (k, v) in self.types.iter() {
            let Some(t) = schema.types.get(k) else {continue;};
            if !t.eq(v) {
                warn!("Overlapping type defined in multiple files. Will be overwritten: {}: {:?} overrides {:?}", k, t, v);
            }
        }
        self.types.extend(schema.types);
    }
}


use super::table::*;


#[derive(Default, Debug, Serialize)]
pub struct SchemaDifference {
    pub added: Vec<TableAdded>,
    pub removed: Vec<TableRemoved>,
    pub changed: Vec<TableChanged>,
}

impl Schema {
    pub fn get_types(&self, mapping: &TypeMapping) -> HashMap<String, String> {
        if &self.type_mapping == mapping {
            return self.types
                .clone()
                .into_iter()
                .map(|(k, v)| {(k, v.inner)})
                .collect()
        }
        self.types
            .clone()
            .into_iter()
            .map(|(k, v)| {(v.inner, k)})
            .collect()
    }

    pub fn change_mappings(&mut self, mapping: TypeMapping) {
        let type_mappings = self.get_types(&mapping);

        for table in self.tables.values_mut() {
            table.map_types(&type_mappings);
        }
        self.types = type_mappings.into_iter().map(|(k, v)| {(k, Type{inner: v})}).collect();
        self.type_mapping = mapping;
    }

    pub fn difference(&self, other: &Self) -> SchemaDifference {
        let mut added = vec![];
        let mut removed = self.tables.clone();
        let mut changed = vec![];
        
        let mut types = self.get_types(&TypeMapping::Pg);
        let other_types = other.get_types(&TypeMapping::Pg);
        types.extend(other_types);

        for (key, value) in other.tables.iter() {
            if let Some(table) = removed.remove(key) {
                table
                    .difference(value)
                    .map(|mut v| {v.map_types(&types); changed.push(v)} );
            } else {
                added.push(TableAdded(value.clone()));
            };
        }
        SchemaDifference{
            added, 
            removed: removed
                .into_iter()
                .map(|(_k, v)| TableRemoved(v))
                .collect(), 
            changed
        }
    }
}




impl RenderScheme for Schema {
    fn render_tables(&self, registry: &handlebars::Handlebars, template_name: &str) -> anyhow::Result<Vec<(String, String)>> {
        let mut data = vec![];
        for (name, table) in self.tables.iter() {
            let rendered = registry.render(template_name, &table)?;
            data.push((name.clone(), rendered));
        }
        Ok(data)
    }

    fn render(&self, registry: &handlebars::Handlebars, template_name: &str) -> anyhow::Result<String> {
        let rendered = registry.render(template_name, &self)?;
        Ok(rendered)
    }
}