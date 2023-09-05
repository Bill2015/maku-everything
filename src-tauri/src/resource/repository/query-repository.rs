use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Thing, thing};

use crate::common::repository::{env, tablens};
use crate::resource::application::dto::ResourceResDto;
use crate::resource::application::dto::ResourceDetailDto;
use crate::resource::infrastructure::ResourceQueryBuilder;

pub static RESOURCE_QUERY_REPOSITORY: ResourceQueryRepository<'_> = ResourceQueryRepository::init(&env::DB);

/**
 * Repository */
 pub struct ResourceQueryRepository<'a> {
    db: &'a Surreal<Client>,
}

impl<'a> ResourceQueryRepository<'a> {
    pub const fn init(db: &'a Surreal<Client>) -> Self {
        ResourceQueryRepository { db: db }
    }

    pub async fn get_all(&self) -> surrealdb::Result<Vec<ResourceResDto>> {
       let sql = r#"
            SELECT 
                *,
                (->belong->category.root_path)[0] as file.root
            FROM type::table($table)"#;

        let mut response = self.db
            .query(sql)
            .bind(("table", &tablens::RESOURCE))
            .await?;

        let result: Vec<ResourceResDto> = response
            .take(0)
            .unwrap();

        Ok(result)
    }

    pub async fn get_by_id(&self, id: &String) -> surrealdb::Result<Option<ResourceResDto>> {
        let sql = r#"
            SELECT 
                *,
                (->belong->category.root_path)[0] as file.root
            FROM type::table($table)
            WHERE id == $id"#;

        let mut response = self.db
            .query("SELECT * FROM type::table($table) WHERE id == $id")
            .bind(("table", &tablens::RESOURCE))
            .bind(("id", thing(id.as_str()).unwrap()))
            .await?;

        let result: Option<ResourceResDto> = response
            .take(0)
            .unwrap_or(None);

        Ok(result) 
    }

    // TODO: need more data to measure response time
    pub async fn detail(&self, id: &String)  -> surrealdb::Result<Option<ResourceDetailDto>> {
        let sql = r#"
            SELECT 
            *,
            (->belong->category.root_path)[0] as file.root,
            (SELECT 
                *,
                (->belong->subject.name)[0] AS subject_name
                FROM tag 
                WHERE ->tagging->resource.id CONTAINS $parent.id
            ) AS tags
            FROM resource
            WHERE id == $id"#;
        
        let mut response = self.db
            .query(sql)
            // .bind(("table", &tablens::RESOURCE))
            .bind(("id", thing(id.as_str()).unwrap()))
            .await?;

        let result: Option<ResourceDetailDto> = response
            .take(0)
            .unwrap();

        Ok(result) 
    }

    pub async fn query(&self, builder: ResourceQueryBuilder) -> surrealdb::Result<Vec<ResourceResDto>> {
        let query_string = builder.build();

        let sql = format!(
            r#"SELECT 
                *,
                (->belong->category.root_path)[0] as file.root
            FROM resource WHERE {}"#
        , query_string);

        let mut response = self.db
            .query(sql)
            .await?;

        let result: Vec<ResourceResDto> = response
            .take(0)
            .unwrap();

        Ok(result) 
    }
}
