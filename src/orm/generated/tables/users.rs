// THIS FILE IS GENERATED, NOT FOR MANUAL EDIT
use sqlx::{Executor, FromRow, Postgres};

use crate::{prelude::{ModelOps, SqlBuilder, TableSelector}};

#[derive(Clone,Debug)]
#[derive(FromRow)]
pub struct Users {
    pub name: String,
    pub t: bigdecimal::BigDecimal,
    pub code: String,
    pub created_by: String,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

impl TableSelector for Users {
    const TABLE_NAME: &'static str = "users";
    const TABLE_SCHEMA: &'static str = "auth";
    type TypePK = String;
    fn pk_column() -> &'static str {
        "name"
    }
    fn columns() -> &'static [&'static str] {
        &[
            "name", 
            "t", 
            "code", 
            "created_by", 
            "updated_by", 
            "updated_at", 
            "created_at", 
        ]
    }
}

#[cfg(feature="postgres")]
impl ModelOps<sqlx::Postgres> for Users 
{
    async fn insert<'e, E>(&self, exec: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = sqlx::Postgres>,
        for<'q> <sqlx::Postgres as sqlx::Database>::Arguments<'q>: Default + sqlx::IntoArguments<'q, sqlx::Postgres>,
    {
        let sql = SqlBuilder::<Postgres, Self>::insert_on_conflict_update();
        sqlx::query(&sql)
            .bind(&self.name)
            .bind(&self.t)
            .bind(&self.code)
            .bind(&self.created_by)
            .bind(&self.updated_by)
            .bind(&self.updated_at)
            .bind(&self.created_at)
            .execute(exec)
            .await?;
        Ok(())
    }

    async fn select<'e, E>(exec: E) -> Result<Vec<Self>, sqlx::Error>
    where
        E: Executor<'e, Database = sqlx::Postgres>,
        Self: for<'r> FromRow<'r, <sqlx::Postgres as sqlx::Database>::Row>
    {
        let sql = SqlBuilder::<Postgres, Users>::select_all();
        let recs = sqlx::query_as::<_, Users>(&sql)
            .fetch_all(exec)
            .await?;
        Ok(recs)
    }

    async fn select_by_pk<'e, E>(
        pk: Self::TypePK,
        exec: E,
    ) -> Result<Option<Self>, sqlx::Error>
    where
        E: Executor<'e, Database = sqlx::Postgres>,
        Self: for<'r> FromRow<'r, <sqlx::Postgres as sqlx::Database>::Row>,
    {
        let sql = SqlBuilder::<Postgres, Users>::select_by_pk();
        let rec = sqlx::query_as::<_, Users>(&sql)
            .bind(pk)
            .fetch_optional(exec)
            .await?;
        Ok(rec)
    }
    
    async fn delete<'e, E>(exec: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = sqlx::Postgres>
    {
        let sql = SqlBuilder::<Postgres, Users>::delete_all();
        sqlx::query(&sql)
            .execute(exec)
            .await?;
        Ok(())
    }
    
    async fn delete_by_pk<'e, E>(pk: Self::TypePK, exec: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = sqlx::Postgres>
    {
        let sql = SqlBuilder::<Postgres, Users>::delete_by_pk();
        sqlx::query(&sql)
            .bind(pk)
            .execute(exec)
            .await?;
        Ok(())
    }
    
    async fn count<'e, E>(exec: E) -> Result<i64, sqlx::Error>
    where
        E: Executor<'e, Database = sqlx::Postgres> {
        use sqlx::Row;
        let sql = SqlBuilder::<Postgres, Users>::count();
        let rec = sqlx::query(&sql)
            .fetch_one(exec)
            .await?;
        Ok(rec.get(0))
    }
}

#[cfg(feature="mysql")]
impl ModelOps<sqlx::MySql> for Users 
{
    async fn insert<'e, E>(&self, exec: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = sqlx::MySql>,
        for<'q> <sqlx::MySql as sqlx::Database>::Arguments<'q>: Default + sqlx::IntoArguments<'q, sqlx::MySql>,
    {
        let sql = SqlBuilder::<Postgres, Self>::insert_on_conflict_update();
        sqlx::query(&sql)
            .bind(&self.name)
            .bind(&self.t)
            .bind(&self.code)
            .bind(&self.created_by)
            .bind(&self.updated_by)
            .bind(&self.updated_at)
            .bind(&self.created_at)
            .execute(exec)
            .await?;
        Ok(())
    }

    async fn select<'e, E>(exec: E) -> Result<Vec<Self>, sqlx::Error>
    where
        E: Executor<'e, Database = sqlx::MySql>,
        Self: for<'r> FromRow<'r, <sqlx::MySql as sqlx::Database>::Row>
    {
        let sql = SqlBuilder::<Postgres, Users>::select_all();
        let recs = sqlx::query_as::<_, Users>(&sql)
            .fetch_all(exec)
            .await?;
        Ok(recs)
    }

    async fn select_by_pk<'e, E>(
        pk: Self::TypePK,
        exec: E,
    ) -> Result<Option<Self>, sqlx::Error>
    where
        E: Executor<'e, Database = sqlx::MySql>,
        Self: for<'r> FromRow<'r, <sqlx::MySql as sqlx::Database>::Row>,
    {
        let sql = SqlBuilder::<Postgres, Users>::select_by_pk();
        let rec = sqlx::query_as::<_, Users>(&sql)
            .bind(pk)
            .fetch_optional(exec)
            .await?;
        Ok(rec)
    }
    
    async fn delete<'e, E>(exec: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = sqlx::MySql>
    {
        let sql = SqlBuilder::<Postgres, Users>::delete_all();
        sqlx::query(&sql)
            .execute(exec)
            .await?;
        Ok(())
    }
    
    async fn delete_by_pk<'e, E>(pk: Self::TypePK, exec: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = sqlx::MySql>
    {
        let sql = SqlBuilder::<Postgres, Users>::delete_by_pk();
        sqlx::query(&sql)
            .bind(pk)
            .execute(exec)
            .await?;
        Ok(())
    }
    
    async fn count<'e, E>(exec: E) -> Result<i64, sqlx::Error>
    where
        E: Executor<'e, Database = sqlx::MySql> {
        use sqlx::Row;
        let sql = SqlBuilder::<Postgres, Users>::count();
        let rec = sqlx::query(&sql)
            .fetch_one(exec)
            .await?;
        Ok(rec.get(0))
    }
}

#[cfg(feature="sqlite")]
impl ModelOps<sqlx::Sqlite> for Users 
{
    async fn insert<'e, E>(&self, exec: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = sqlx::Sqlite>,
        for<'q> <sqlx::Sqlite as sqlx::Database>::Arguments<'q>: Default + sqlx::IntoArguments<'q, sqlx::Sqlite>,
    {
        let sql = SqlBuilder::<Postgres, Self>::insert_on_conflict_update();
        sqlx::query(&sql)
            .bind(&self.name)
            .bind(&self.t)
            .bind(&self.code)
            .bind(&self.created_by)
            .bind(&self.updated_by)
            .bind(&self.updated_at)
            .bind(&self.created_at)
            .execute(exec)
            .await?;
        Ok(())
    }

    async fn select<'e, E>(exec: E) -> Result<Vec<Self>, sqlx::Error>
    where
        E: Executor<'e, Database = sqlx::Sqlite>,
        Self: for<'r> FromRow<'r, <sqlx::Sqlite as sqlx::Database>::Row>
    {
        let sql = SqlBuilder::<Postgres, Users>::select_all();
        let recs = sqlx::query_as::<_, Users>(&sql)
            .fetch_all(exec)
            .await?;
        Ok(recs)
    }

    async fn select_by_pk<'e, E>(
        pk: Self::TypePK,
        exec: E,
    ) -> Result<Option<Self>, sqlx::Error>
    where
        E: Executor<'e, Database = sqlx::Sqlite>,
        Self: for<'r> FromRow<'r, <sqlx::Sqlite as sqlx::Database>::Row>,
    {
        let sql = SqlBuilder::<Postgres, Users>::select_by_pk();
        let rec = sqlx::query_as::<_, Users>(&sql)
            .bind(pk)
            .fetch_optional(exec)
            .await?;
        Ok(rec)
    }
    
    async fn delete<'e, E>(exec: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = sqlx::Sqlite>
    {
        let sql = SqlBuilder::<Postgres, Users>::delete_all();
        sqlx::query(&sql)
            .execute(exec)
            .await?;
        Ok(())
    }
    
    async fn delete_by_pk<'e, E>(pk: Self::TypePK, exec: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = sqlx::Sqlite>
    {
        let sql = SqlBuilder::<Postgres, Users>::delete_by_pk();
        sqlx::query(&sql)
            .bind(pk)
            .execute(exec)
            .await?;
        Ok(())
    }
    
    async fn count<'e, E>(exec: E) -> Result<i64, sqlx::Error>
    where
        E: Executor<'e, Database = sqlx::Sqlite> {
        use sqlx::Row;
        let sql = SqlBuilder::<Postgres, Users>::count();
        let rec = sqlx::query(&sql)
            .fetch_one(exec)
            .await?;
        Ok(rec.get(0))
    }
}
