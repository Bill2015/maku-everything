

use once_cell::sync::Lazy;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

use crate::modules::common::repository::env;

pub static PRE_DEFINED_REPOSITORY: PreDefinedRepository<'_> = PreDefinedRepository::init(&env::DB);

pub mod sql_utils {
    use chrono::NaiveDate;

    use super::sql_predefn;

    const DATE_FORAMTER: &str = "%Y-%m-%d";

    pub fn sql_range_date(field_name: &str, value: (&Option<NaiveDate>, &Option<NaiveDate>)) -> String {
        if let (Some(start), Some(end)) = value {
            return sql_predefn::between(
                field_name, 
                format!("type::datetime('{}')", start.format(DATE_FORAMTER)), 
                format!("type::datetime('{}')", end.format(DATE_FORAMTER)),
            );
        }
        if let Some(start) = value.0 {
            return format!("({} >= type::datetime('{}'))", field_name, start.format(DATE_FORAMTER))
        }
        if let Some(end) = value.1 {
            return format!("({} <= type::datetime('{}'))", field_name, end.format(DATE_FORAMTER))
        }
        return "(true)".to_string();
    }

    pub fn sql_range_number(field_name: &str, value: (&Option<usize>, &Option<usize>)) -> String {
        match value {
            (Some(start), Some(end)) => {
                sql_predefn::between(field_name, start, end)
            },
            (Some(start), None) => {
                format!("({} >= {})", field_name, start)
            },
            (None, Some(end)) => {
                return format!("({} <= {})", field_name, end)
            }
            _ => "(true)".to_string()
        }
    }

    /// It will generated the
    /// ```rust
    /// assert_eq!("!(person.married == true)", sql_with_prefix("(person.married == true)"))
    /// ```
    pub fn sql_with_prefix<S: Into<String>>(not_flag: bool, s: S) -> String {
        match not_flag {
            true => format!("!{}", s.into()),
            false => s.into(),
        }
    }

    /// It will generated the
    /// ```rust
    /// assert_eq!("(person.married == true)", sql_equal("person.married", true))
    /// ```
    pub fn sql_equal<S: ToString>(field_name: &str, target: &S) -> String {
        format!("({} == {})", field_name, target.to_string())
    }

    /// It will generated the
    /// ```rust
    /// assert_eq!("(person.emails CONTAINS .com)", sql_contain("person.emails", ".com"))
    /// ```
    pub fn sql_contain<S: Into<String>>(field_name: &str, target: S) -> String {
        format!("({} CONTAINS {})", field_name, target.into())
    }

    /// It will generated the
    /// ```rust
    /// assert_eq!("(fn::to_lowercase(person.name) CONTAINS string::lowercase(\"john\"))", sql_contain_string("person.name", "john"))
    /// ```
    pub fn sql_contain_string<S: Into<String>>(field_name: &str, target: S) -> String {
        format!(r#"({} CONTAINS string::lowercase("{}"))"#, sql_predefn::to_lowercase(field_name), target.into())
    }
}

pub mod sql_predefn {
    use std::fmt::Display;

    pub const BETWEEN_FN: &str = "fn::between";
    pub const LOWERCASE_FN: &str = "fn::to_lowercase";

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

    /// generate sql pre-defined `to_lowercase` function
    /// ### example:
    /// ```
    /// let s1 = between("abc");
    /// 
    /// assert_eq!(s1, "fn::to_lowercase(abc)")
    /// ```
    pub fn to_lowercase<T: ToString + Display>(target: T) -> String {
        format!("({}({}))", LOWERCASE_FN, target)
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
        self.define_to_lowercase().await?;

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

    async fn define_to_lowercase(&self) -> surrealdb::Result<()> {
        let sql = format!(r#"
            DEFINE FUNCTION {0}($name: any) {{
                RETURN IF type::is::string($name) THEN
                    string::lowercase($name)
                ELSE
                    ""
                END
            }};
        "#, sql_predefn::LOWERCASE_FN);

        let _ = self.db
            .query(sql)
            .await?;

        Ok(())
    }
}
