impl crate::prelude::SqlGen for sqlx::MySql {
    fn placeholder(_i: usize) -> String {
        return "?".to_owned()
    }
}

impl crate::prelude::OrmDB for sqlx::MySql {}