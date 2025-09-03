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


pub const MOD_TEMPLATE : &str = include_str!("../templates/tables/mod.hbr");
pub const DB_TABLES_TEMPLATE : &str = include_str!("../templates/db_tables.hbr");
pub const TABLE_TEMPLATE : &str = include_str!("../templates/tables/table.hbr");

pub fn generate<P: AsRef<std::path::Path>>(schema : &Schema, out_dir: P) -> Result<(), Box<dyn std::error::Error>>{
    
    let mut reg = handlebars::Handlebars::new();

    reg.register_template_string("mod_template", MOD_TEMPLATE)?;
    reg.register_template_string("db_tables_template", DB_TABLES_TEMPLATE)?;
    reg.register_template_string("table_template", TABLE_TEMPLATE)?;
    reg.register_schema_reader_helpers();

    let tables_mod = schema.render(&reg, "mod_template")?;
    let db_tables = schema.render(&reg, "db_tables_template")?;

    let dbs = &[
        DBFeature{feature: "postgres", db: "sqlx::Postgres"},
        DBFeature{feature: "mysql", db: "sqlx::MySql"},
        DBFeature{feature: "sqlite", db: "sqlx::Sqlite"},
    ];

    std::fs::create_dir(out_dir.as_ref().join("tables")).ok();


    std::fs::write(out_dir.as_ref().join("tables/mod.rs"), tables_mod)?;
    std::fs::write(out_dir.as_ref().join("db_tables.rs"), db_tables)?;

    
    for (name, table) in schema.tables.iter() {
        let table = TableDBs{table, dbs};
        let rendered = reg.render("table_template", &table)?;
        std::fs::write(out_dir.as_ref()
            .join("tables")
            .join(name)
            .with_extension("rs"), rendered)?;
    }

    Ok(())
}