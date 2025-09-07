use crate::prelude::OrmDB;

#[derive(Clone)]
pub struct Orm<E : Clone>
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

    pub fn get_executor<'a, 'b>(&'a self) -> &'b sqlx::Pool<DB> where 'a: 'b { &self.executor }

    pub async fn begin_tx(&self) -> Result<OrmTX<DB>, Box<dyn std::error::Error>> {
        let v = self.executor.begin().await?;
        Ok(OrmTX { inner: v })
    }
}


pub struct OrmTX<DB: OrmDB> {
    pub inner: sqlx::Transaction<'static, DB>,
}

impl<DB: OrmDB> OrmTX<DB> {
    pub fn get_inner(&mut self) -> &mut <DB as sqlx::Database>::Connection { &mut *self.inner }

    pub async fn commit(self) -> Result<(), sqlx::Error> {
        self.inner.commit().await
    }
    pub async fn rollback(self) -> Result<(), sqlx::Error> {
        self.inner.rollback().await
    }
}

