use std::{cell::RefCell, fmt::Debug, ops::Deref, rc::Rc, sync::Arc};

use futures_core::{future::BoxFuture, stream::BoxStream};
use sqlx::Executor;

use crate::prelude::OrmDB;


pub struct Orm<E>
{
    pub(crate) executor: E
}

impl<DB : OrmDB> Orm<sqlx::Pool<DB>>
{
    pub fn new(pool: sqlx::Pool<DB>) -> Self {
        Self { 
            executor: pool,
        }
    }
    pub async fn begin_tx(&self) -> Result<Orm<TXInner<DB>>, Box<dyn std::error::Error>> {
        // let v = self.executor.begin().await?.d;


        todo!()
        // Ok(OrmTX { executor: RefCell::new(v) })
    }
}

#[derive(Debug)]
pub struct TXInner<'a, DB: OrmDB> {
    pub executor: Arc<RefCell<sqlx::Transaction<'a, DB>>>
}

impl<'x, DB> Executor<'x> for &'x TXInner<'x, DB>
where
    &'x TXInner<'x, DB>: Debug + Send + Sized,
    // E: Executor<'e, Database = DB>,
    DB: OrmDB,
{
    type Database = DB;
    
    fn fetch_many<'e, 'q: 'e, E>(
        self,
        query: E,
    ) -> BoxStream<
        'e,
        Result<
            sqlx::Either<<Self::Database as sqlx::Database>::QueryResult, <Self::Database as sqlx::Database>::Row>,
            sqlx::Error,
        >,
    >
    where
        'e: 'e,
        E: 'q + sqlx::Execute<'q, Self::Database> {
        let b = self.executor.borrow_mut();
        todo!()
    }
    
    fn fetch_optional<'e, 'q: 'e, E>(
        self,
        query: E,
    ) -> BoxFuture<'e, Result<Option<<Self::Database as sqlx::Database>::Row>, sqlx::Error>>
    where
        'e: 'e,
        E: 'q + sqlx::Execute<'q, Self::Database> {
        todo!()
    }
    
    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        parameters: &'e [<Self::Database as sqlx::Database>::TypeInfo],
    ) -> BoxFuture<'e, Result<<Self::Database as sqlx::Database>::Statement<'q>, sqlx::Error>>
    where
        'e: 'e {
        todo!()
    }
    
    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> BoxFuture<'e, Result<sqlx::Describe<Self::Database>, sqlx::Error>>
    where
        'e: 'e {
        todo!()
    }

}





// pub struct OrmTX<E>
// {
//     pub executor: RefCell<E>
// }

// impl<DB: OrmDB> OrmTX<sqlx::Transaction<'_, DB>> {
//     pub async fn commit(self) -> Result<(), Box<dyn std::error::Error>> {
//         self.executor.into_inner().commit().await?;
//         Ok(())
//     }
//     pub async fn rollback(self) -> Result<(), Box<dyn std::error::Error>> {
//         self.executor.into_inner().rollback().await?;
//         Ok(())
//     }
// }

