use std::{borrow::Cow, marker::PhantomData, sync::Arc};

use sqlx::{Executor, FromRow, IntoArguments, query::{Query, QueryAs}};

use crate::prelude::OrmDB;

pub struct ColumnDef {
    pub name: &'static str,
    pub nullable: bool,
    pub is_unique: bool,
    pub default: Option<&'static str>,
    pub is_primary: bool
}

pub trait TableSelector {
    const TABLE_NAME: &'static str;
    const TABLE_SCHEMA: &'static str;
    type TypePK;
    fn columns() -> &'static [ColumnDef];
    fn pk_column() -> &'static str;
    // fn get_field<T, DB>(&self, name: &str) -> &Option<T> where DB: OrmDB, T: for<'t> sqlx::Encode<'t, DB> + sqlx::Type<DB>;
    fn is_field_set(&self, field_name: &str) -> bool;
}

pub enum SaveMode {
    Insert,
    Update,
    Upsert
}

pub use SaveMode::*;

#[async_trait::async_trait]
pub trait ModelOps<DB>: Sized + TableSelector
where
    DB: OrmDB,
    Self::NonActive : for<'r> FromRow<'r, <DB as sqlx::Database>::Row>,
{
    type NonActive;
    fn save<'e, E>(self, exec: E, mode: SaveMode) -> impl std::future::Future<Output = Result<Option<Self::NonActive>, sqlx::Error>> + Send
    where
        E: Executor<'e, Database = DB>,
        for<'q> <DB as sqlx::Database>::Arguments<'q>: Default + sqlx::IntoArguments<'q, DB>
        ;
    fn complete_query<'s, 'q, T>(&'s self, q: QueryAs<'q, DB, T, <DB as sqlx::Database>::Arguments<'q>>)
    -> sqlx::query::QueryAs<'q,sqlx::Postgres,T, <sqlx::Postgres as sqlx::Database> ::Arguments<'q> > where 's: 'q;
    fn insert<'e, E>(self, exec: E) -> impl std::future::Future<Output = Result<Option<Self::NonActive>, sqlx::Error>> + Send
    where
        E: Executor<'e, Database = DB>,
        for<'q> <DB as sqlx::Database>::Arguments<'q>: Default + sqlx::IntoArguments<'q, DB>
        ;
    fn update<'e, E>(self, exec: E) -> impl std::future::Future<Output = Result<Option<Self::NonActive>, sqlx::Error>> + Send
    where
        E: Executor<'e, Database = DB>,
        for<'q> <DB as sqlx::Database>::Arguments<'q>: Default + sqlx::IntoArguments<'q, DB>
        ;
    fn upsert<'e, E>(self, exec: E) -> impl std::future::Future<Output = Result<Option<Self::NonActive>, sqlx::Error>> + Send
    where
        E: Executor<'e, Database = DB>,
        for<'q> <DB as sqlx::Database>::Arguments<'q>: Default + sqlx::IntoArguments<'q, DB>
        ;
    fn select_by_pk<'e, E>(pk: &Self::TypePK, exec: E) -> impl std::future::Future<Output = Result<Option<Self::NonActive>, sqlx::Error>> + Send
    where
        E: Executor<'e, Database = DB>,
        Self: for<'r> FromRow<'r, <DB as sqlx::Database>::Row>
        ;
    fn delete_by_pk<'e, E>(pk: &Self::TypePK, exec: E) -> impl std::future::Future<Output = Result<Option<Self::NonActive>, sqlx::Error>> + Send
    where
        E: Executor<'e, Database = DB>;
    fn count<'e, E>(exec: E) -> impl std::future::Future<Output = Result<i64, sqlx::Error>> + Send
    where
        E: Executor<'e, Database = DB>;
}


pub struct DBSelector<'e, DB, E, T>
where
    T: TableSelector,
    DB: OrmDB,
    &'e E: Executor<'e, Database = DB>,
{
    pub(crate) _g: PhantomData<DB>,
    pub(crate) _t: PhantomData<T>,
    pub(crate) q_src: String,
    pub(crate) executor: &'e E
}

pub struct DBSelectorInteraction<'q, 'e, DB, Ex, Out>
where
    DB: OrmDB,
    &'e Ex: Executor<'e, Database = DB>,
{
    pub(crate) q: QueryAs<'q, DB, Out, <DB as sqlx::Database>::Arguments<'q>>,
    pub(crate) executor: &'e Ex
}

impl<'e, DB, E, T> DBSelector<'e, DB, E, T>
where
    T: TableSelector + ModelOps<DB> + for<'r> FromRow<'r, <DB as sqlx::Database>::Row>,
    DB: OrmDB,
    for<'a> <DB as sqlx::Database>::Arguments<'a>: IntoArguments<'a, DB>,
    &'e E: Executor<'e, Database = DB>,
{
    pub fn new(executor: &'e E) -> Self {
        Self {
            q_src: Default::default(),
            _g: PhantomData,
            _t: PhantomData,
            executor
        }
    }
    
    pub fn save(self, data: T, mode: SaveMode) -> impl std::future::Future<Output = Result<Option<<T as ModelOps<DB>>::NonActive>, sqlx::Error>> + Send {
        data.save(self.executor, mode)
    }

    pub fn select<'q>(&'e mut self, query: &str) -> DBSelectorInteraction<'q, 'e, DB, E, T>
    where 
        'e: 'q, 
        for<'r> T: FromRow<'r, <DB as sqlx::Database>::Row>
    {   
        self.interaction_builder("select", query)
    }

    pub fn delete<'q>(&'e mut self, query: &str) -> DBSelectorInteraction<'q, 'e, DB, E, T>
    where 
        'e: 'q, 
        for<'r> T: FromRow<'r, <DB as sqlx::Database>::Row>
    {
        self.interaction_builder("delete", query)
    }

    fn interaction_builder<'q>(&'e mut self, prefix: &str, query: &str) -> DBSelectorInteraction<'q, 'e, DB, E, T>
    where 
        'e: 'q, 
        for<'r> T: FromRow<'r, <DB as sqlx::Database>::Row>
    {
        let q_src = if query.to_ascii_lowercase().trim().starts_with(prefix) {
            query.to_string()
        } else {
            format!("{} * from {}.{} {}", prefix, T::TABLE_SCHEMA, T::TABLE_NAME, query)
        };
        self.q_src = q_src;
        DBSelectorInteraction {
            q: sqlx::query_as::<DB, T>(&self.q_src),
            executor: self.executor
        }
    }
    
    pub fn select_by_pk(self, key: &T::TypePK) -> impl std::future::Future<Output = Result<Option<<T as ModelOps<DB>>::NonActive>, sqlx::Error>> + Send {
        T::select_by_pk(key, self.executor)
    }
    pub fn delete_by_pk(self, key: &T::TypePK) -> impl std::future::Future<Output = Result<Option<<T as ModelOps<DB>>::NonActive>, sqlx::Error>> + Send {
        T::delete_by_pk(key, self.executor)
    }
    pub fn count(self) -> impl std::future::Future<Output = Result<i64, sqlx::Error>> + Send {
        T::count(self.executor)
    }
}


impl<'q, 'e, DB, Ex, Out> DBSelectorInteraction<'q, 'e, DB, Ex, Out>
where 
    DB: OrmDB,
    <DB as sqlx::Database>::Arguments<'q>: IntoArguments<'q, DB>,
    &'e Ex: Executor<'e, Database = DB>,
{
    pub fn bind<V>(mut self, value: V) -> Self 
    where 
        V: 'q + sqlx::Encode<'q, DB> + sqlx::Type<DB>
    {
        self.q = self.q.bind(value);
        self
    }
    pub async fn fetch(self) -> Result<Vec<Out>, sqlx::Error>
    where
        Out: Send + Unpin + for<'r> FromRow<'r, <DB as sqlx::Database>::Row>
    {
        self.q.fetch_all(self.executor).await
    }
}






//todo!
pub struct TxSelectorInteractions<'e, DB, T>
where
    T: TableSelector,
    DB: OrmDB,
{
    pub(crate) _g: PhantomData<DB>,
    pub(crate) _t: PhantomData<T>,
    pub(crate) q: Option<Query<'e, DB, <DB as sqlx::Database>::Arguments<'e>>>,
    pub(crate) executor: Option<&'e mut <DB as sqlx::Database>::Connection>
}

impl<'e, DB, T> TxSelectorInteractions<'e, DB, T>
where
    T: TableSelector + for<'r> FromRow<'r, <DB as sqlx::Database>::Row> + ModelOps<DB>,
    DB: OrmDB,
    for<'a> <DB as sqlx::Database>::Arguments<'a>: IntoArguments<'a, DB>,
    &'e mut <DB as sqlx::Database>::Connection: Executor<'e, Database = DB>,
{
    pub fn new(executor: &'e mut <DB as sqlx::Database>::Connection) -> Self {
        Self {
            q: None,
            _g: PhantomData,
            _t: PhantomData,
            executor: Some(executor)
        }
    }
    pub fn save(mut self, data: T, mode: SaveMode) -> impl std::future::Future<Output = Result<Option<<T as ModelOps<DB>>::NonActive>, sqlx::Error>> + Send {
        data.save(self.executor.take().unwrap(), mode)
    }
    pub fn select_by_pk(mut self, key: &T::TypePK) -> impl std::future::Future<Output = Result<Option<<T as ModelOps<DB>>::NonActive>, sqlx::Error>> + Send {
        T::select_by_pk(key, self.executor.take().unwrap())
    }
    pub fn delete_by_pk(mut self, key: &T::TypePK) -> impl std::future::Future<Output = Result<Option<<T as ModelOps<DB>>::NonActive>, sqlx::Error>> + Send {
        T::delete_by_pk(key, self.executor.take().unwrap())
    }
    pub fn count(mut self) -> impl std::future::Future<Output = Result<i64, sqlx::Error>> + Send {
        T::count(self.executor.take().unwrap())
    }
}

