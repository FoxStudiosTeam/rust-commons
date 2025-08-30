use crate::prelude::{SqlGen};

pub trait OrmDB : sqlx::Database + SqlGen {}