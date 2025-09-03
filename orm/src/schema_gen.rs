use std::{
    fs::{self, DirEntry},
};

use schema_reader::prelude::*;

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

const MOD_TEMPLATE: &str = include_str!("../templates/tables/mod.hbr");
const MOD_TEMPLATE_NAME: &str = "mod_template";

const DB_TABLES_TEMPLATE: &str = include_str!("../templates/db_tables.hbr");
const DB_TABLES_TEMPLATE_NAME: &str = "db_tables_template";

const TABLE_TEMPLATE: &str = include_str!("../templates/tables/table.hbr");
const TABLE_TEMPLATE_NAME: &str = "table_template";

const MIGRATION_TEMPLATE: &str =
    include_str!("../templates/migrations/initial_table_migration.hbr");
const MIGRATION_TEMPLATE_NAME: &str = "migration_template";

pub fn generate<P: AsRef<std::path::Path>>(
    schema: &Schema,
    out_dir: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut reg = handlebars::Handlebars::new();

    reg.register_template_string(MOD_TEMPLATE_NAME, MOD_TEMPLATE)?;
    reg.register_template_string(DB_TABLES_TEMPLATE_NAME, DB_TABLES_TEMPLATE)?;
    reg.register_template_string(TABLE_TEMPLATE_NAME, TABLE_TEMPLATE)?;
    reg.register_template_string(MIGRATION_TEMPLATE_NAME, MIGRATION_TEMPLATE)?;
    reg.register_schema_reader_helpers();

    let tables_mod = schema.render(&reg, MOD_TEMPLATE_NAME)?;
    let db_tables = schema.render(&reg, DB_TABLES_TEMPLATE_NAME)?;

    let dbs = &[
        DBFeature {
            feature: "postgres",
            db: "sqlx::Postgres",
        },
        DBFeature {
            feature: "mysql",
            db: "sqlx::MySql",
        },
        DBFeature {
            feature: "sqlite",
            db: "sqlx::Sqlite",
        },
    ];

    std::fs::create_dir(out_dir.as_ref().join("tables"))?;

    std::fs::write(out_dir.as_ref().join("tables/mod.rs"), tables_mod)?;
    std::fs::write(out_dir.as_ref().join("db_tables.rs"), db_tables)?;

    let raw = fs::read_dir(out_dir.as_ref())?
        .into_iter()
        .filter_map(|v| v.ok())
        .collect::<Vec<DirEntry>>();
    if let Some(max) = max_value(&raw) {
        for (name, table) in schema.tables.iter() {
            let table = TableDBs { table, dbs };
            let rendered = reg.render(TABLE_TEMPLATE_NAME, &table)?;
            std::fs::write(
                out_dir
                    .as_ref()
                    .join("tables")
                    .join(name)
                    .with_extension("rs"),
                rendered,
            )?;

            let migration = reg.render(MIGRATION_TEMPLATE_NAME, &table.table)?;
            if let Some(rw_ver) = check_is_exists(name.as_ref(), &raw) {
                let mut ver = rw_ver;
                let mut nm = "creating".to_string();
                if max > rw_ver {
                    ver +=1;
                    nm = "updating".to_string();
                }
                std::fs::write(
                    out_dir
                        .as_ref()
                        .join(format!("migrations/V{}__{}_{}.sql", ver, nm, name)),
                    migration,
                )?;
            }
        }
    }

    Ok(())
}

fn check_is_exists(name: &str, entries: &Vec<DirEntry>) -> Option<i32> {
    for entry in entries {
        let path = entry.path();

        if let Some(file_name) = path.file_name()?.to_str() {
            if file_name.contains(name) {
                if let Ok(num) = extract_digits(file_name).parse::<i32>() {
                    return Some(num);
                }
            }
        }
    }

    None
}

fn max_value(entries: &Vec<DirEntry>) -> Option<i32> {
    let mut max_value: Option<i32> = None;

    for entry in entries {
        let path = entry.path();

        if let Some(file_name) = path.file_name()?.to_str() {
            if let Ok(num) = extract_digits(file_name).parse::<i32>() {
                max_value = Some(match max_value {
                    Some(current_max) => current_max.max(num),
                    None => num,
                });
            }
        }
    }

    max_value
}

fn extract_digits(s: &str) -> String {
    s.chars().filter(|c| c.is_ascii_digit()).collect()
}
