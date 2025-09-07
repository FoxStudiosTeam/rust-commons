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


pub const MOD_TEMPLATE : &str = include_str!("../templates/mod.hbr");
pub const TABLE_TEMPLATE : &str = include_str!("../templates/table.hbr");

pub fn generate<P: AsRef<std::path::Path>>(schema : &Schema, out_dir: P) -> Result<(), Box<dyn std::error::Error>>{
    
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