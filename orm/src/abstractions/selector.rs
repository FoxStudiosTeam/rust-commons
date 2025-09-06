use std::marker::PhantomData;

use sqlx::{Executor, FromRow, IntoArguments};

use crate::prelude::OrmDB;

pub trait TableSelector {
    const TABLE_NAME: &'static str;
    const TABLE_SCHEMA: &'static str;
    type TypePK;
    fn columns() -> &'static [&'static str];
    fn pk_column() -> &'static str;
}

#[async_trait::async_trait]
pub trait ModelOps<DB>: Sized + TableSelector
where
    DB: OrmDB,
{
    /// Returns `true` if the record was inserted, `false` if primary key already exists
    fn insert<'e, E>(self, exec: E) -> impl std::future::Future<Output = Result<bool, sqlx::Error>> + Send
    where
        E: Executor<'e, Database = DB>,
        for<'q> <DB as sqlx::Database>::Arguments<'q>: Default + sqlx::IntoArguments<'q, DB>
        ;
    fn insert_update<'e, E>(self, exec: E) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + Send
    where
        E: Executor<'e, Database = DB>,
        for<'q> <DB as sqlx::Database>::Arguments<'q>: Default + sqlx::IntoArguments<'q, DB>
        ;
    fn select<'e, E>(exec: E) -> impl std::future::Future<Output = Result<Vec<Self>, sqlx::Error>> + Send
    where
        E: Executor<'e, Database = DB>,
        Self: for<'r> FromRow<'r, <DB as sqlx::Database>::Row>
        ;
    fn select_by_pk<'e, E>(pk: &Self::TypePK, exec: E) -> impl std::future::Future<Output = Result<Option<Self>, sqlx::Error>> + Send
    where
        E: Executor<'e, Database = DB>,
        Self: for<'r> FromRow<'r, <DB as sqlx::Database>::Row>
        ;
    fn delete<'e, E>(exec: E) -> impl std::future::Future<Output = Result<bool, sqlx::Error>> + Send
    where
        E: Executor<'e, Database = DB>;
    fn delete_by_pk<'e, E>(pk: &Self::TypePK, exec: E) -> impl std::future::Future<Output = Result<bool, sqlx::Error>> + Send
    where
        E: Executor<'e, Database = DB>;
    fn count<'e, E>(exec: E) -> impl std::future::Future<Output = Result<i64, sqlx::Error>> + Send
    where
        E: Executor<'e, Database = DB>;
}


pub struct SelectorInteractions<'e, DB, E, T>
where
    T: TableSelector,
    DB: OrmDB,
    &'e E: Executor<'e, Database = DB>,
{
    pub(crate) _g: PhantomData<DB>,
    pub(crate) _t: PhantomData<T>,
    pub(crate) executor: &'e E
}

impl<'e, DB, E, T> SelectorInteractions<'e, DB, E, T>
where
    T: TableSelector + ModelOps<DB> + for<'r> FromRow<'r, <DB as sqlx::Database>::Row>,
    DB: OrmDB,
    for<'a> <DB as sqlx::Database>::Arguments<'a>: IntoArguments<'a, DB>,
    &'e E: Executor<'e, Database = DB>,
{
    pub fn new(executor: &'e E) -> Self {
        Self {
            _g: PhantomData,
            _t: PhantomData,
            executor
        }
    }
    pub fn insert(self, data: T) -> impl std::future::Future<Output = Result<bool, sqlx::Error>> + Send {
        data.insert(self.executor)
    }
    pub fn insert_update(self, data: T) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + Send {
        data.insert_update(self.executor)
    }
    pub fn select(self) -> impl std::future::Future<Output = Result<Vec<T>, sqlx::Error>> + Send {
        T::select(self.executor)
    }
    pub fn select_by_pk(self, key: &T::TypePK) -> impl std::future::Future<Output = Result<Option<T>, sqlx::Error>> + Send {
        T::select_by_pk(key, self.executor)
    }
    pub fn delete(self) -> impl std::future::Future<Output = Result<bool, sqlx::Error>> + Send {
        T::delete(self.executor)
    }
    pub fn delete_by_pk(self, key: &T::TypePK) -> impl std::future::Future<Output = Result<bool, sqlx::Error>> + Send {
        T::delete_by_pk(key, self.executor)
    }
    pub fn count(self) -> impl std::future::Future<Output = Result<i64, sqlx::Error>> + Send {
        T::count(self.executor)
    }
}

pub struct TxSelectorInteractions<'e, DB, T>
where
    T: TableSelector,
    DB: OrmDB,
{
    pub(crate) _g: PhantomData<DB>,
    pub(crate) _t: PhantomData<T>,
    pub(crate) executor:  &'e mut <DB as sqlx::Database>::Connection
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
            _g: PhantomData,
            _t: PhantomData,
            executor: executor
        }
    }
    pub fn insert(self, data: T) -> impl std::future::Future<Output = Result<bool, sqlx::Error>> + Send {
        data.insert(self.executor)
    }
    pub fn insert_update(self, data: T) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + Send {
        data.insert_update(self.executor)
    }
    pub fn select(self) -> impl std::future::Future<Output = Result<Vec<T>, sqlx::Error>> + Send {
        T::select(self.executor)
    }
    pub fn select_by_pk(self, key: &T::TypePK) -> impl std::future::Future<Output = Result<Option<T>, sqlx::Error>> + Send {
        T::select_by_pk(key, self.executor)
    }
    pub fn delete(self) -> impl std::future::Future<Output = Result<bool, sqlx::Error>> + Send {
        T::delete(self.executor)
    }
    pub fn delete_by_pk(self, key: &T::TypePK) -> impl std::future::Future<Output = Result<bool, sqlx::Error>> + Send {
        T::delete_by_pk(key, self.executor)
    }
    pub fn count(self) -> impl std::future::Future<Output = Result<i64, sqlx::Error>> + Send {
        T::count(self.executor)
    }
}

