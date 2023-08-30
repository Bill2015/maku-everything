#[path ="./query-repository.rs"]
mod query;
pub use query::{RESOURCE_QUERY_REPOSITORY,ResourceQueryRepository};

use serde::{Deserialize, Serialize};
use serde_json::error::Category;
use surrealdb::{Surreal, sql};
use surrealdb::sql::{Datetime, Thing, Value, thing};
use surrealdb::engine::remote::ws::Client;

use crate::common::infrastructure::IRepoMapper;
use crate::common::repository::env;
use crate::common::repository::relatens;
use crate::common::repository::tablens;
use crate::resource::domain::ResourceAggregate;
use crate::resource::infrastructure::ResourceRepoMapper;

pub static RESOURCE_REPOSITORY: ResourceRepository<'_> = ResourceRepository::init(&env::DB);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceFileDo {
    pub uuid: String,
    pub name: String,
    pub path: String,
    pub ext: String,
}

/**
 * Resource Data Object */
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceDO {
    #[serde(skip_serializing)]
    pub id: Thing,
    pub title: String,
    pub description: String,
    pub file: Option<ResourceFileDo>,
    pub auth: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,

    #[serde(skip_serializing)]
    #[serde(default = "default_resource")]
    pub belong_category: String,

    #[serde(skip_serializing)]
    #[serde(default = "default_vec")]
    pub tags: Vec<String>,
}

fn default_resource() -> String {
    "/".to_string()
}

fn default_vec() -> Vec<String> {
    Vec::new()
}
/**
 * Repository */
pub struct ResourceRepository<'a> {
    db: &'a Surreal<Client>,
}

impl<'a> ResourceRepository<'a> {
    pub const fn init(db: &'a Surreal<Client>) -> Self {
        ResourceRepository { db: db }
    }

    async fn return_aggregate_by_id(&self, id: String) -> surrealdb::Result<Option<ResourceAggregate>> {
        let sql = "SELECT *, type::string((->resource_belong.out)[0]) AS belong_category FROM type::table($table) WHERE id == $id";

        let mut response = self.db
            .query(sql)
            .bind(("table", tablens::RESOURCE))
            .bind(("id", thing(id.as_str()).unwrap()))
            .await?;

        let result: Vec<ResourceDO> = response
            .take(0)?;

        let item = result
            .first();


        let aggregate = match item {
            Some(value) => Some(ResourceRepoMapper::do_to_aggregate(value.clone())),
            None => None,
        };

        Ok(aggregate)
    }

    async fn create_belong_category_relation(&self, self_id: &String, category_id: &String) -> surrealdb::Result<()> {
        let sql: String = format!("RELATE $resource->{}->$category", relatens::RESOURCE_BELONG);
        let _ = self.db
            .query(sql)
            .bind(("resource", thing(self_id).unwrap()))
            .bind(("category", thing(category_id).unwrap()))
            .await?;
        Ok(())
    }

    pub async fn is_exist(&self, id: String) -> bool {
        let thing_id = thing(id.as_str()).unwrap();
        let result: Option<ResourceDO> = self.db
            .select(thing_id)
            .await
            .unwrap_or(None);

        match result {
            Some(value) => true,
            None => false,
        }
    }

    pub async fn find_by_id(&self, id: String) -> surrealdb::Result<Option<ResourceAggregate>> {
        let result = self.return_aggregate_by_id(id)
            .await?;

        Ok(result)
    }

    pub async fn save(&self, data: ResourceAggregate) -> surrealdb::Result<ResourceAggregate> {
        let resource_do = ResourceRepoMapper::aggregate_to_do(data);
        let id: Thing = resource_do.id.clone();
        
        let belong_category = resource_do.belong_category.clone();

        let is_new: bool = id.id.to_raw().is_empty();

        // save data
        let result: Option<ResourceDO> = match is_new {
            true => {
                // let db auto generate the id
                self.db
                    .create(tablens::RESOURCE)
                    .content(resource_do)
                    .await?

            }
            false => {
                self.db
                    .update(id)
                    .content(resource_do)
                    .await?
            }
        };

        let new_id = (&result).as_ref().unwrap().id.to_string();
        // create relation
        if is_new == true {
            self.create_belong_category_relation(&new_id, &belong_category)
                .await?;
        }

        let final_result = self.return_aggregate_by_id(new_id)
            .await?;

        Ok(final_result.unwrap())
    }

    pub async fn delete(&self, id: String) -> surrealdb::Result<Option<ResourceAggregate>> {
        let thing_id = thing(id.as_str()).unwrap();
        let result: Option<ResourceDO> = self.db
            .delete(thing_id)
            .await?;

        let aggregate: Option<ResourceAggregate> = match result {
            Some(value) => Some(ResourceRepoMapper::do_to_aggregate(value)),
            None => None,
        };

        Ok(aggregate)
    }
}

