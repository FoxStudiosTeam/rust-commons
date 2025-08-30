// THIS FILE IS GENERATED, NOT FOR MANUAL EDIT
use std::marker::PhantomData;
use sqlx::Executor;

use crate::prelude::{Orm, OrmDB, SelectorInteractions};

use crate::prelude::tables::{
    Users,
};

pub trait DBTables<DB, E> 
where
    DB: OrmDB,
{
    fn users<'e>(&'e self) -> SelectorInteractions<'e, DB, E, Users>
    where 
        &'e E: Executor<'e, Database = DB>;
}

impl<DB, E> DBTables<DB, E> for Orm<E>
where
    DB: OrmDB,
{
    fn users<'e>(&'e self) -> SelectorInteractions<'e, DB, E, Users>
    where
        &'e E: Executor<'e, Database = DB>,
    {
        SelectorInteractions {
            _g: PhantomData,
            _t: PhantomData,
            executor: &self.executor,
        }
    }
}


