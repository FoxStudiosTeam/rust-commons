impl crate::prelude::SqlGen for sqlx::Sqlite {
    fn placeholder(_i: usize) -> String {
        return "?".to_owned()
    }
}

impl crate::prelude::OrmDB for sqlx::Postgres {}
