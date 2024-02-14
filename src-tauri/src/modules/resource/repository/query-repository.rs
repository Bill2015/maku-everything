use once_cell::sync::Lazy;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::thing;

use crate::modules::common::infrastructure::QueryBuilderResult;
use crate::modules::common::repository::{env, tablens};
use crate::modules::resource::application::dto::ResourceResDto;
use crate::modules::resource::application::query::ResourceDetailDto;
use crate::modules::resource::infrastructure::ResourceStringQL;

pub static RESOURCE_QUERY_REPOSITORY: ResourceQueryRepository<'_> = ResourceQueryRepository::init(&env::DB);

/**
 * Repository */
 pub struct ResourceQueryRepository<'a> {
    db: &'a Lazy<Surreal<Client>>,
}

impl<'a> ResourceQueryRepository<'a> {
    const ROOT_PATH_FIELD: &str = "(->belong->category.root_path)[0] as root_path";

    pub const fn init(db: &'a Lazy<Surreal<Client>>) -> Self {
        ResourceQueryRepository { db: db }
    }

    pub async fn get_all(&self) -> surrealdb::Result<Vec<ResourceResDto>> {
        let sql = format!(r#"
            SELECT
                *,
                {}
            FROM type::table($table)"#, 
            Self::ROOT_PATH_FIELD);

        let  result: Vec<ResourceResDto> = self.db
            .query(sql)
            .bind(("table", &tablens::RESOURCE))
            .await?
            .take(0)?;

        Ok(result)
    }

    pub async fn get_by_id(&self, id: &String) -> surrealdb::Result<Option<ResourceResDto>> {
        let sql = format!(r#"
            SELECT
                *,
                {}
            FROM type::table($table)
            WHERE id == $id"#, 
            Self::ROOT_PATH_FIELD);

        let result: Option<ResourceResDto>  = self.db
            .query(sql)
            .bind(("table", &tablens::RESOURCE))
            .bind(("id", thing(id.as_str()).unwrap()))
            .await?
            .take(0)
            .unwrap_or(None);

        Ok(result) 
    }

    // TODO: need more data to measure response time
    pub async fn detail(&self, id: &String)  -> surrealdb::Result<Option<ResourceDetailDto>> {
        let sql = format!(r#"
            SELECT 
            *,
            {},
            (SELECT 
                *,
                (->belong->subject.name)[0] AS subject_name,
                (->tagging.added_at)[0] AS added_at,
                array::len(->tagging.out) as tagged_count
                FROM tag 
                WHERE ->tagging->resource.id CONTAINS $parent.id
                ORDER BY added_at ASC
            ) AS tags
            FROM type::table($table)
            WHERE id == $id"#, 
            Self::ROOT_PATH_FIELD);
                    
        let result: Option<ResourceDetailDto> = self.db
            .query(sql)
            .bind(("table", &tablens::RESOURCE))
            .bind(("id", thing(id.as_str()).unwrap()))
            .await?
            .take(0)?;

        Ok(result) 
    }

    pub async fn query(&self, builder_result: QueryBuilderResult) -> surrealdb::Result<Vec<ResourceResDto>> {
        let sql = format!(
            r#"SELECT 
                *,
                {}
            FROM type::table($table) WHERE {query_string}"#, 
            Self::ROOT_PATH_FIELD, query_string = builder_result.to_string());

        let result: Vec<ResourceResDto> = self.db
            .query(sql)
            .bind(("table", &tablens::RESOURCE))
            .await?
            .take(0)?;

        Ok(result) 
    }

    pub async fn string_ql(&self, ql: ResourceStringQL) -> surrealdb::Result<Vec<ResourceResDto>> {
        let query = ql.get();
        
        let sql = format!(
            r#"SELECT 
                *,
                {}
            FROM type::table($table) WHERE {query}"#,
            Self::ROOT_PATH_FIELD, query = query);

        let result: Vec<ResourceResDto> = self.db
            .query(sql)
            .bind(("table", &tablens::RESOURCE))
            .await?
            .take(0)?;

        Ok(result)
    }
}
