use once_cell::sync::Lazy;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::thing;

use crate::modules::common::infrastructure::QueryBuilderResult;
use crate::modules::common::repository::env;
use crate::tag::application::dto::TagResDto;

pub static TAG_QUERY_REPOSITORY: TagQueryRepository<'_> = TagQueryRepository::init(&env::DB);

pub struct TagQueryRepository<'a> {
    db: &'a Lazy<Surreal<Client>>,
}
impl<'a> TagQueryRepository<'a> {
    const SUBJECT_NAME_FIELD: &str = "(->belong->subject.name)[0] as subject_name";
    const CATEGORY_NAME_FIELD: &str = "(->belong->category.name)[0] as category_name";
    const TAGGED_COUNT_FIELD: &str = "array::len(->tagging.out) as tagged_count";

    pub const fn init(db: &'a Lazy<Surreal<Client>>) -> Self {
        TagQueryRepository { db: db }
    }

    pub async fn get_all(&self) -> surrealdb::Result<Vec<TagResDto>> {
        let sql = format!(r#"
            SELECT 
                *,
                {},
                {},
                {}
            FROM tag"#,
            Self::SUBJECT_NAME_FIELD,
            Self::CATEGORY_NAME_FIELD,
            Self::TAGGED_COUNT_FIELD);

        let mut response = self.db
            .query(sql)
            .await?;

        let result: Vec<TagResDto> = response
            .take(0)
            .unwrap();

        Ok(result)
    }

    pub async fn get_by_id(&self, id: &String) -> surrealdb::Result<Option<TagResDto>> {
        let sql = format!(r#"
            "SELECT 
                *,
                {},
                {},
                {}
            FROM tag WHERE id == $id"#,
            Self::SUBJECT_NAME_FIELD,
            Self::CATEGORY_NAME_FIELD,
            Self::TAGGED_COUNT_FIELD);

        let mut response = self.db
            .query(sql)
            .bind(("id", thing(id.as_str()).unwrap()))
            .await?;

        let result: Option<TagResDto> = response
            .take(0)
            .unwrap_or(None);

        Ok(result) 
    }

    pub async fn query(&self, builder_result: QueryBuilderResult) -> surrealdb::Result<Vec<TagResDto>> {
        let sql = format!(r#"
            SELECT 
                *,
                {},
                {},
                {}
            FROM tag WHERE {query_string}"#, 
            Self::SUBJECT_NAME_FIELD,
            Self::CATEGORY_NAME_FIELD,
            Self::TAGGED_COUNT_FIELD,
            query_string = builder_result.to_string());

        let mut response = self.db
            .query(sql)
            .await?;

        let result: Vec<TagResDto> = response
            .take(0)
            .unwrap();

        Ok(result) 
    }
}