impl crate::prelude::SqlGen for sqlx::Postgres {
    fn placeholder(i: usize) -> String {
        format!("${}", i + 1)
    }
}

impl crate::prelude::OrmDB for sqlx::Postgres {}