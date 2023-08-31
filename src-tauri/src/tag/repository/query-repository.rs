use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Thing, thing};

use crate::common::repository::{env, tablens};
use crate::tag::application::dto::TagResDto;

pub static TAG_QUERY_REPOSITORY: TagQueryRepository<'_> = TagQueryRepository::init(&env::DB);

pub struct TagQueryRepository<'a> {
    db: &'a Surreal<Client>,
}
impl<'a> TagQueryRepository<'a> {
    pub const fn init(db: &'a Surreal<Client>) -> Self {
        TagQueryRepository { db: db }
    }

    pub async fn get_all(&self) -> surrealdb::Result<Vec<TagResDto>> {
        let mut response = self.db
            .query("SELECT * FROM type::table($table)")
            .bind(("table", &tablens::TAG))
            .await?;

        let result: Vec<TagResDto> = response
            .take(0)
            .unwrap();

        Ok(result)
    }

    pub async fn get_by_id(&self, id: &String) -> surrealdb::Result<Option<TagResDto>> {
        let mut response = self.db
            .query("SELECT * FROM tag WHERE id == $id")
            .bind(("id", thing(id.as_str()).unwrap()))
            .await?;

        let result: Option<TagResDto> = response
            .take(0)
            .unwrap_or(None);

        Ok(result) 
    }
}