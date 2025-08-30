use std::{cell::RefCell, rc, sync::Arc};

use chrono::NaiveDateTime;
use simple_orm::prelude::*;
use sqlx::{postgres::PgPoolOptions, *};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let pg = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://testuser:testpass@localhost:5432/testdb")
        .await
        .unwrap();

    let mut tx = pg.begin().await.unwrap();
    let v = Arc::new(RefCell::new(Some(tx)));

    // let v = &mut **v.borrow_mut();
    let mut a = v.borrow_mut();
    let inner = a.take().unwrap();
    inner.commit().await.unwrap();
    // let v = v.take().into_inner();






    sqlx::query("SELECT 1").execute(&pg).await?;

    // let mut e = pg.begin().await?;
    // let ex = &mut *e;


    let orm = Orm::new(pg);


    let user = Users{
        name: "aboba".to_string(),
        t: 1234.into(),
        code: "test".to_string(),
        created_by: "test".to_string(),
        updated_by: "test".to_string(),
        created_at: NaiveDateTime::default(),
        updated_at: NaiveDateTime::default(),
    };


    orm.users().insert(user).await.unwrap();
    let r = orm.users();
    
    let r = r.select_by_pk("test".to_string()).await.unwrap();

    tracing::info!("got: {:?}", r);
    tracing::info!("count: {:?}", orm.users().count().await.unwrap());
    Ok(())
}


