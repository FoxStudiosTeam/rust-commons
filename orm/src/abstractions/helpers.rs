use std::marker::PhantomData;

use crate::prelude::{OrmDB, TableSelector};

pub trait SqlGen {
    fn placeholder(i: usize) -> String;
    fn full_table_name(schema: &str, table: &str) -> String {
        format!(r#""{}"."{}""#, schema, table)
    }
}

pub struct SqlBuilder<DB: OrmDB, T: TableSelector> {
    _db: PhantomData<DB>,
    _t: PhantomData<T>,
}

impl<DB: OrmDB, T: TableSelector> SqlBuilder<DB, T> {
    pub fn insert() -> String {
        let table = DB::full_table_name(T::TABLE_SCHEMA, T::TABLE_NAME);
        let cols = T::columns();
        let placeholders: Vec<String> = (0..cols.len())
            .map(|i| DB::placeholder(i))
            .collect();
        format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table,
            cols.join(", "),
            placeholders.join(", ")
        )
    }

    pub fn insert_on_conflict_update() -> String {
        let table = DB::full_table_name(T::TABLE_SCHEMA, T::TABLE_NAME);
        let cols = T::columns();
        let placeholders: Vec<String> = (0..cols.len())
            .map(|i| DB::placeholder(i))
            .collect();

        let updates: Vec<String> = cols.iter()
            .filter(|&&c| c != T::pk_column()) 
            .map(|c| format!("{} = EXCLUDED.{}", c, c))
            .collect();

        format!(
            "INSERT INTO {} ({}) VALUES ({}) ON CONFLICT ({}) DO UPDATE SET {}",
            table,
            cols.join(", "),
            placeholders.join(", "),
            T::pk_column(),
            updates.join(", ")
        )
    }


    pub fn select_all() -> String {
        let table = DB::full_table_name(T::TABLE_SCHEMA, T::TABLE_NAME);
        format!("SELECT {} FROM {}", T::columns().join(", "), table)
    }

    pub fn select_by_pk() -> String {
        let table = DB::full_table_name(T::TABLE_SCHEMA, T::TABLE_NAME);
        format!(
            "SELECT {} FROM {} WHERE {} = {}",
            T::columns().join(", "),
            table,
            T::pk_column(),
            DB::placeholder(0),
        )
    }

    pub fn delete_all() -> String {
        let table = DB::full_table_name(T::TABLE_SCHEMA, T::TABLE_NAME);
        format!("DELETE FROM {}", table)
    }

    pub fn delete_by_pk() -> String {
        let table = DB::full_table_name(T::TABLE_SCHEMA, T::TABLE_NAME);
        format!(
            "DELETE FROM {} WHERE {} = {}",
            table,
            T::pk_column(),
            DB::placeholder(0),
        )
    }

    pub fn count() -> String {
        let table = DB::full_table_name(T::TABLE_SCHEMA, T::TABLE_NAME);
        format!("SELECT COUNT(*) as cnt FROM {}", table)
    }
}
