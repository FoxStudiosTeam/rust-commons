use dao_generator::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = dao_generator::prelude::Schema::from_dir("./schemas")?;
    generate(&schema)?;
    Ok(())
}

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

fn generate(schema : &Schema) -> Result<(), Box<dyn std::error::Error>>{
    let tables_mod = schema.render("./templates/tables/mod.hbr")?;
    let db_tables = schema.render("./templates/db_tables.hbr")?;

    let dbs = &[
        DBFeature{feature: "postgres", db: "sqlx::Postgres"},
        DBFeature{feature: "mysql", db: "sqlx::MySql"},
        DBFeature{feature: "sqlite", db: "sqlx::Sqlite"},
    ];

    std::fs::remove_dir_all("./src/generated/tables/")?;
    std::fs::create_dir("./src/generated/tables/")?;


    std::fs::write("./src/generated/tables/mod.rs", tables_mod)?;
    std::fs::write("./src/generated/db_tables.rs", db_tables)?;

    
    let template = std::fs::read_to_string("./templates/tables/table.hbr")?;

    let mut reg = handlebars::Handlebars::new();
    reg.register_default_helpers();

    for (name, table) in schema.tables.iter() {
        let table = TableDBs{table, dbs};
        let rendered = reg.render_template(&template, &table)?;
        std::fs::write(format!("./src/generated/tables/{}.rs", name), rendered)?;
    }

    Ok(())
}