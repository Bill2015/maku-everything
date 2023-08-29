use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Thing, thing};

use crate::common::repository::{env, tablens};
use crate::resource::application::dto::ResourceResDto;

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
        let mut response = self.db
            .query("SELECT * FROM type::table($table)")
            .bind(("table", &tablens::RESOURCE))
            .await?;

        let result: Vec<ResourceResDto> = response
            .take(0)
            .unwrap();

        Ok(result)
    }

    pub async fn get_by_id(&self, id: &String) -> surrealdb::Result<Option<ResourceResDto>> {
        let mut response = self.db
            .query("SELECT * FROM type::table($table) WHERE id == $id")
            .bind(("table", &tablens::RESOURCE))
            .bind(("id", thing(id.as_str())))
            .await?;

        let result: Option<ResourceResDto> = response
            .take(0)
            .unwrap_or(None);

        Ok(result) 
    }
}
