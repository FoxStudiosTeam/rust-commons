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
        let v = self.executor.begin().await?;
        Ok(Orm { executor: TXInner { inner: v }})
    }
}


pub struct TXInner<'a, DB: OrmDB> {
    pub inner: sqlx::Transaction<'a, DB>,
}

impl<'a, DB: OrmDB> TXInner<'a, DB> {
    pub async fn commit(self) -> Result<(), sqlx::Error> {
        self.inner.commit().await
    }
    pub async fn rollback(self) -> Result<(), sqlx::Error> {
        self.inner.rollback().await
    }
}

