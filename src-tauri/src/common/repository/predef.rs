

use once_cell::sync::Lazy;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

use crate::common::repository::env;

pub static PRE_DEFINED_REPOSITORY: PreDefinedRepository<'_> = PreDefinedRepository::init(&env::DB);


pub mod sql_predefn {
    use std::fmt::Display;

    pub const BETWEEN_FN: &str = "fn::between";

    /// generate sql pre-defined `between` function
    /// ### example:
    /// ```
    /// let s1 = between("10", 0, 9);
    /// let s2 = between(23, 50, 45);
    /// 
    /// assert_eq!(s1, "fn::between(10, 0, 9)")
    /// assert_eq!(s1, "fn::between(23, 50, 45)")
    /// ```
    pub fn between<T, F>(target: T, start: F, end: F) -> String 
        where
            T: ToString + Display,
            F: ToString + Display,
    {
        format!("({}({}, {}, {}))", BETWEEN_FN, target, start, end)
    }
}

///
/// This repository only can pre-defined some `SurrealDB` function \
/// Do not add other SQL logic inside \
/// See Alos
///   - https://docs.surrealdb.com/docs/surrealql/statements/define/function
pub struct PreDefinedRepository<'a> {
    db: &'a Lazy<Surreal<Client>>,
}

impl<'a> PreDefinedRepository<'a> {
    pub const fn init(db: &'a Lazy<Surreal<Client>>) -> Self {
        Self { db }
    }

    pub async fn define_fns(&self) -> surrealdb::Result<()> {
        self.define_between_fn().await?;

        Ok(())
    }

    /// define between fn, for range used
    async fn define_between_fn(&self) -> surrealdb::Result<()> {
        let sql = format!(r#"
            DEFINE FUNCTION {0}($target: any, $start: any, $end: any) {{
                RETURN ($target >= $start AND $target <= $end);
            }};
        "#, sql_predefn::BETWEEN_FN);

        let _ = self.db
            .query(sql)
            .await?;

        Ok(())
    }
}
