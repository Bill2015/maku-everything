use once_cell::sync::Lazy;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::thing;

use crate::modules::common::infrastructure::QueryBuilderResult;
use crate::modules::common::repository::{env, tablens};
use crate::modules::subject::application::dto::SubjectResDto;

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

    pub async fn query(&self, builder_result: QueryBuilderResult) -> surrealdb::Result<Vec<SubjectResDto>> {
        let sql = format!(r#"
            SELECT 
                *
            FROM subject WHERE {}"#, 
            builder_result.to_string());

        let mut response = self.db
            .query(sql)
            .await?;

        let result: Vec<SubjectResDto> = response
            .take(0)
            .unwrap();

        Ok(result) 
    }
}