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


pub trait ModelOps<DB>: Sized + TableSelector
where
    DB: OrmDB,
{
    fn insert<'e, E>(&self, exec: E) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + Send
    where
        E: Executor<'e, Database = DB>,
        for<'q> <DB as sqlx::Database>::Arguments<'q>: Default + sqlx::IntoArguments<'q, DB>;
    fn select<'e, E>(exec: E) -> impl std::future::Future<Output = Result<Vec<Self>, sqlx::Error>> + Send
    where
        E: Executor<'e, Database = DB>,
        Self: for<'r> FromRow<'r, <DB as sqlx::Database>::Row>;
    fn select_by_pk<'e, E>(pk: Self::TypePK, exec: E) -> impl std::future::Future<Output = Result<Option<Self>, sqlx::Error>> + Send
    where
        E: Executor<'e, Database = DB>,
        Self: for<'r> FromRow<'r, <DB as sqlx::Database>::Row>;
    fn delete<'e, E>(exec: E) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + Send
    where
        E: Executor<'e, Database = DB>;
    fn delete_by_pk<'e, E>(pk: Self::TypePK, exec: E) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + Send
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
    pub async fn insert(&self, data: T) -> Result<(), Box<dyn std::error::Error>> {
        Ok(data.insert(self.executor).await?)
    }
    pub async fn select(&self) -> Result<Vec<T>, Box<dyn std::error::Error>> {
        Ok(T::select(self.executor).await?)
    }
    pub async fn select_by_pk(&self, key: T::TypePK) -> Result<Option<T>, Box<dyn std::error::Error>> {
        Ok(T::select_by_pk(key, self.executor).await?)
    }
    pub async fn delete(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(T::delete(self.executor).await?)
    }
    pub async fn delete_by_pk(&self, key: T::TypePK) -> Result<(), Box<dyn std::error::Error>> {
        Ok(T::delete_by_pk(key, self.executor).await?)
    }
    pub async fn count(&self) -> Result<i64, Box<dyn std::error::Error>> {
        Ok(T::count(self.executor).await?)
    }
}




// pub struct TxSelectorInteractions<'e, DB, T>
// where
//     T: TableSelector,
//     DB: OrmDB,
// {
//     pub(crate) _g: PhantomData<DB>,
//     pub(crate) _t: PhantomData<T>,
//     pub(crate) executor: &'e crate::prelude::TXInner<'e, DB>
// }


// impl<'e, DB, T> TxSelectorInteractions<'e, DB, T>
// where
//     T: TableSelector + ModelOps<DB> + for<'r> FromRow<'r, <DB as sqlx::Database>::Row>,
//     DB: OrmDB,
//     for<'a> <DB as sqlx::Database>::Arguments<'a>: IntoArguments<'a, DB>,
//     &'e mut <DB as sqlx::Database>::Connection: Executor<'e, Database = DB>,
// {
//     pub async fn insert(self, data: T) -> Result<(), Box<dyn std::error::Error>> {
//         let mut e = &mut **self.executor.inner.borrow_mut();
//         let res = data.insert(e).await?;
//         Ok(res)
//     }
//     pub async fn select(&self) -> Result<Vec<T>, Box<dyn std::error::Error>> {
//         Ok(T::select(self.executor).await?)
//     }
//     pub async fn select_by_pk(&self, key: T::TypePK) -> Result<Option<T>, Box<dyn std::error::Error>> {
//         Ok(T::select_by_pk(key, self.executor).await?)
//     }
//     pub async fn delete(&self) -> Result<(), Box<dyn std::error::Error>> {
//         Ok(T::delete(self.executor).await?)
//     }
//     pub async fn delete_by_pk(&self, key: T::TypePK) -> Result<(), Box<dyn std::error::Error>> {
//         Ok(T::delete_by_pk(key, self.executor).await?)
//     }
//     pub async fn count(&self) -> Result<i64, Box<dyn std::error::Error>> {
//         Ok(T::count(self.executor).await?)
//     }
// }
