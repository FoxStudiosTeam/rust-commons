use schema_reader::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(serde::Serialize)]
struct TableDBs<'a> {
    table: &'a Table,
    dbs: &'static [DBFeature],
}

#[derive(serde::Serialize)]
struct DBFeature {
    feature: &'static str,
    db: &'static str,
}


pub const MOD_TEMPLATE : &str = include_str!("../templates/mod.hbr");
pub const TABLE_TEMPLATE : &str = include_str!("../templates/table.hbr");

pub const MIGRATION_TEMPLATE : &str = include_str!("../templates/migration.hbr");

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct LatestMigrationState {
    latest: usize,
    state: Schema,
}

pub fn generate_migration<P: AsRef<std::path::Path>>(mut schema : Schema, out_dir: P, migration_name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let mut reg = handlebars::Handlebars::new();
    reg.register_template_string("migration_template", MIGRATION_TEMPLATE)?;
    schema.change_mappings(TypeMapping::Pg);
    let state_path = out_dir.as_ref().join("latest").with_extension("migration");
    
    let mut prev_state = std::fs::read(&state_path)
        .ok()
        .and_then(|v| 
            bincode::serde::decode_from_slice::<LatestMigrationState, _>(&v, bincode::config::standard()).ok()
                .map(|v|v.0)
        ).unwrap_or_default();
    
    if schema == prev_state.state {
        tracing::info!("No changes in schema");
        return Ok(());
    }

    prev_state.latest += 1;
    let diff = prev_state.state.difference(&schema);
    prev_state.state = schema;
    let rendered = reg.render("migration_template", &diff)?;
    std::fs::write(out_dir.as_ref().join(format!("V{}__{}.sql", prev_state.latest, migration_name.unwrap_or("migration"))), rendered)?;
    tracing::info!("Migration generated!");
    let encoded = bincode::serde::encode_to_vec(&prev_state, bincode::config::standard()).unwrap();
    std::fs::write(state_path, encoded)?;
    tracing::info!("Latest state saved!");
    Ok(())
}

pub fn generate_rust_bindings<P: AsRef<std::path::Path>>(schema : &Schema, out_dir: P) -> Result<(), Box<dyn std::error::Error>>{
    let mut reg = handlebars::Handlebars::new();

    reg.register_template_string("mod_template", MOD_TEMPLATE)?;
    reg.register_template_string("table_template", TABLE_TEMPLATE)?;
    reg.register_schema_reader_helpers();

    let tables_mod = schema.render(&reg, "mod_template")?;

    let dbs = &[
        DBFeature{feature: "postgres", db: "sqlx::Postgres"},
        DBFeature{feature: "mysql", db: "sqlx::MySql"},
        DBFeature{feature: "sqlite", db: "sqlx::Sqlite"},
    ];

    std::fs::write(out_dir.as_ref().join("mod.rs"), tables_mod)?;
    
    for (name, table) in schema.tables.iter() {
        let table = TableDBs{table, dbs};
        let rendered = reg.render("table_template", &table)?;
        std::fs::write(out_dir.as_ref()
            .join(name)
            .with_extension("rs"), rendered)?;
    }
    
    Ok(())
}