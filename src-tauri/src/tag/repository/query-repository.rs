use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Thing, thing};

use crate::common::repository::{env, tablens};
use crate::tag::application::dto::TagResDto;
use crate::tag::infrastructure::TagQueryBuilder;

pub static TAG_QUERY_REPOSITORY: TagQueryRepository<'_> = TagQueryRepository::init(&env::DB);

pub struct TagQueryRepository<'a> {
    db: &'a Lazy<Surreal<Client>>,
}
impl<'a> TagQueryRepository<'a> {
    pub const fn init(db: &'a Lazy<Surreal<Client>>) -> Self {
        TagQueryRepository { db: db }
    }

    pub async fn get_all(&self) -> surrealdb::Result<Vec<TagResDto>> {
        let sql = r#"
            SELECT 
                *,
                (->belong->subject.name)[0] as subject_name,
                (->belong->category.name)[0] as category_name,
                array::len(->tagging.out) as tag_nums
            FROM tag
        "#;

        let mut response = self.db
            .query(sql)
            .await?;

        let result: Vec<TagResDto> = response
            .take(0)
            .unwrap();

        Ok(result)
    }

    pub async fn get_by_id(&self, id: &String) -> surrealdb::Result<Option<TagResDto>> {
        let sql = r#"
            "SELECT 
                *,
                (->belong->subject.name)[0] as subject_name,
                (->belong->category.name)[0] as category_name,
                array::len(->tagging.out) as tag_nums
            FROM tag WHERE id == $id
        "#;

        let mut response = self.db
            .query(sql)
            .bind(("id", thing(id.as_str()).unwrap()))
            .await?;

        let result: Option<TagResDto> = response
            .take(0)
            .unwrap_or(None);

        Ok(result) 
    }

    pub async fn query(&self, builder: TagQueryBuilder) -> surrealdb::Result<Vec<TagResDto>> {
        let query_string = builder.build();

        let sql = format!(
            r#"SELECT 
                *,
                (->belong->subject.name)[0] as subject_name,
                (->belong->category.name)[0] as category_name,
                array::len(->tagging.out) as tag_nums
            FROM tag WHERE {}"#
        , query_string);

        dbg!(&sql);

        let mut response = self.db
            .query(sql)
            .await?;

        dbg!(&response);

        let result: Vec<TagResDto> = response
            .take(0)
            .unwrap();

        Ok(result) 
    }
}