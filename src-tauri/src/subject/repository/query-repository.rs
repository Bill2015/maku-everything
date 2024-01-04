use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Thing, thing};

use crate::common::repository::{env, tablens};
use crate::subject::application::dto::SubjectResDto;
use crate::subject::infrastructure::SubjectQueryBuilder;

pub static SUBJECT_QUERY_REPOSITORY: SubjectQueryRepository<'_> = SubjectQueryRepository::init(&env::DB);

pub struct SubjectQueryRepository<'a> {
    db: &'a Lazy<Surreal<Client>>,
}
impl<'a> SubjectQueryRepository<'a> {
    pub const fn init(db: &'a Lazy<Surreal<Client>>) -> Self {
        SubjectQueryRepository { db: db }
    }

    pub async fn get_all(&self) -> surrealdb::Result<Vec<SubjectResDto>> {
        let mut response = self.db
            .query("SELECT * FROM type::table($table)")
            .bind(("table", &tablens::SUBJECT))
            .await?;

        let result: Vec<SubjectResDto> = response
            .take(0)
            .unwrap();

        Ok(result)
    }

    pub async fn get_by_id(&self, id: &String) -> surrealdb::Result<Option<SubjectResDto>> {
        let mut response = self.db
            .query("SELECT * FROM subject WHERE id == $id")
            .bind(("id", thing(id.as_str())))
            .await?;

        let result: Option<SubjectResDto> = response
            .take(0)
            .unwrap_or(None);

        Ok(result) 
    }

    pub async fn query(&self, builder: SubjectQueryBuilder) -> surrealdb::Result<Vec<SubjectResDto>> {
        let query_string = builder.build();

        let sql = format!(
            r#"SELECT 
                *
            FROM subject WHERE {}"#
        , query_string);

        dbg!(&sql);

        let mut response = self.db
            .query(sql)
            .await?;

        dbg!(&response);

        let result: Vec<SubjectResDto> = response
            .take(0)
            .unwrap();

        Ok(result) 
    }
}