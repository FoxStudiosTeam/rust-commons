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


pub struct TXInner<'a, DB: OrmDB> {
    pub inner: RefCell<sqlx::Transaction<'a, DB>>,
}

impl<'a, DB: OrmDB> TXInner<'a, DB> {
    pub async fn commit(self) -> Result<(), sqlx::Error> {
        // let v  = &mut **self.inner.borrow_mut();
        // drop(v);
        self.inner.into_inner().commit().await
    }

    pub async fn rollback(self) -> Result<(), sqlx::Error> {
        self.inner.into_inner().rollback().await
    }
}

