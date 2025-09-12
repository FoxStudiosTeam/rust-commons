use std::fmt::Display;

use sqlx::{Database, Decode, Type, TypeInfo, ValueRef};

use crate::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct Null;

#[derive(Clone, Debug, Default)]
// pub struct Optional<T>(Option<T>);
pub enum Optional<T>{
    Set(T),
    #[default]
    NotSet
}

impl<T> Optional<T> {
    pub fn is_set(&self) -> bool {
        match self {
            Optional::Set(_) => true,
            Optional::NotSet => false
        }
    }
    pub fn as_option(&self) -> Option<&T> { match self { Optional::Set(t) => Some(t), Optional::NotSet => None } }
    pub fn into_option(self) -> Option<T> { match self { Optional::Set(t) => Some(t), Optional::NotSet => None } }
    pub fn is_none(&self) -> bool { !self.is_set() }
}


pub use Optional::*;

impl<'r, DB, T> Decode<'r, DB> for Optional<T>
where
    DB: Database,
    T: Decode<'r, DB>,
{
    fn decode(value: <DB as Database>::ValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        if value.is_null() {
            Ok(Optional::NotSet)
        } else {
            Ok(Optional::Set(T::decode(value)?))
        }
    }
}

impl<T: Type<DB>, DB: Database> Type<DB> for Optional<T> {
    fn type_info() -> DB::TypeInfo {
        <T as Type<DB>>::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        ty.is_null() || <T as Type<DB>>::compatible(ty)
    }
}



pub trait SqlGen {
    fn placeholder(i: usize) -> String;
    fn full_table_name(schema: &str, table: &str) -> String {
        format!(r#""{}"."{}""#, schema, table)
    }
}

pub trait SqlBuilder<DB: OrmDB> {
    fn select_by_pk() -> String;
    fn delete_by_pk() -> String;
    fn count() -> String;
    fn insert_for(&self) -> Result<String, OrmError>;
    fn update_for(&self) -> Result<String, OrmError>;
    fn upsert_for(&self) -> Result<String, OrmError>;
}

#[derive(Debug)]
pub enum OrmError {
    MissingValue(&'static str),
    NothingToUpdate,
    MissingPrimaryKey,
    NothingToInsert,
}

impl Display for OrmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrmError::MissingValue(name) => write!(f, "Missing value for {}", name),
            OrmError::NothingToUpdate => write!(f, "Nothing to update"),
            OrmError::MissingPrimaryKey => write!(f, "Missing primary key"),
            OrmError::NothingToInsert => write!(f, "Nothing to insert"),
        }
    }
}

impl std::error::Error for OrmError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

// + for<'r> FromRow<'r, <DB as sqlx::Database>::Row> + for <'r> sqlx::Encode<'r, DB> + for <'r> sqlx::Decode<'r, DB> + sqlx::Type<DB>
impl<T, DB: OrmDB> SqlBuilder<DB> for T
where T: TableSelector
{
    fn insert_for(&self) -> Result<String, OrmError> {
        let table = DB::full_table_name(Self::TABLE_SCHEMA, Self::TABLE_NAME);
        let cols = Self::columns();

        let mut insert_cols = Vec::new();
        let mut placeholders = Vec::new();
        let mut i = 0;
        for col in cols.iter() {
            if !col.nullable && col.default.is_none() && !self.is_field_set(col.name) {
                return Err(OrmError::MissingValue(col.name));
            }

            if self.is_field_set(col.name) || (!col.nullable && col.default.is_none()) {
                insert_cols.push(col.name);
                placeholders.push(DB::placeholder(i));
                i += 1;
            }
        }

        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
            table,
            insert_cols.join(", "),
            placeholders.join(", ")
        );

        Ok(sql)
    }

    fn update_for(&self) -> Result<String, OrmError> {
        let table = DB::full_table_name(Self::TABLE_SCHEMA, Self::TABLE_NAME);
        let cols = Self::columns();

        let mut set_clauses = Vec::new();
        let mut idx = 0;

        let mut pk_col = None;
        let mut pk_idx = 0;
        for col in cols.iter() {
            tracing::info!("Column: {}", col.name);
            if col.is_primary {
                pk_col = Some(col.name);
                pk_idx = idx;
                idx += 1;
                continue;
            }
            if self.is_field_set(col.name) {
                set_clauses.push(format!("{} = {}", col.name, DB::placeholder(idx)));
                idx += 1;
            }
        }

        if set_clauses.is_empty() {
            return Err(OrmError::NothingToUpdate);
        }

        let pk_col = pk_col
            .ok_or(OrmError::MissingPrimaryKey)?;
        if !self.is_field_set(pk_col) {
            return Err(OrmError::MissingValue(pk_col));
        }

        let sql = format!(
            "UPDATE {} SET {} WHERE {} = {} RETURNING *",
            table,
            set_clauses.join(", "),
            pk_col,
            DB::placeholder(pk_idx)
        );

        Ok(sql)
    }

    fn upsert_for(&self) -> Result<String, OrmError> {
        let table = DB::full_table_name(Self::TABLE_SCHEMA, Self::TABLE_NAME);
        let cols = Self::columns();

        let mut insert_cols = Vec::new();
        let mut placeholders = Vec::new();
        let mut update_clauses = Vec::new();
        let mut idx = 0;

        let mut pk_col = None;

        for col in cols.iter() {
            if col.is_primary {
                pk_col = Some(col.name);
            }
            if !col.nullable && col.default.is_none() && !self.is_field_set(col.name) {
                return Err(OrmError::MissingValue(col.name));
            }

            if self.is_field_set(col.name) || (!col.nullable && col.default.is_none()) {
                insert_cols.push(col.name);
                placeholders.push(DB::placeholder(idx));
                idx += 1;
                update_clauses.push(format!("{} = EXCLUDED.{}", col.name, col.name));
            }
        }

        if insert_cols.is_empty() {
            return Err(OrmError::NothingToInsert);
        }
        let pk_col = pk_col.ok_or(OrmError::MissingPrimaryKey)?;
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({}) \
            ON CONFLICT ({}) DO UPDATE SET {} \
            RETURNING *",
            table,
            insert_cols.join(", "),
            placeholders.join(", "),
            pk_col,
            update_clauses.join(", ")
        );

        Ok(sql)
    }


    fn select_by_pk() -> String {
        let table = DB::full_table_name(Self::TABLE_SCHEMA, Self::TABLE_NAME);
        let col_names: Vec<&str> = Self::columns().iter().map(|c| c.name).collect();
        format!(
            "SELECT {} FROM {} WHERE {} = {}",
            col_names.join(", "),
            table,
            Self::pk_column(),
            DB::placeholder(0)
        )
    }

    fn delete_by_pk() -> String {
        let table = DB::full_table_name(Self::TABLE_SCHEMA, Self::TABLE_NAME);
        format!(
            "DELETE FROM {} WHERE {} = {} RETURNING *",
            table,
            Self::pk_column(),
            DB::placeholder(0)
        )
    }

    fn count() -> String {
        let table = DB::full_table_name(Self::TABLE_SCHEMA, Self::TABLE_NAME);
        format!("SELECT COUNT(*) as cnt FROM {}", table)
    }
}
