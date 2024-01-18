use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Thing, Datetime, thing};

use crate::common::infrastructure::IRepoMapper;
use crate::common::repository::{env, tablens};
use crate::category::domain::CategoryAggregate;
use crate::category::infrastructure::CategoryRepoMapper;

pub static CATEGORY_REPOSITORY: CategoryRepository<'_> = CategoryRepository::init(&env::DB);

/**
 * Category Data Object */
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryDO {
    #[serde(skip_serializing)]
    pub id: Thing,
    pub name: String,
    pub description: String,
    pub auth: bool,
    pub root_path: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}
/**
 * Repository */
pub struct CategoryRepository<'a> {
    db: &'a Lazy<Surreal<Client>>,
}

impl<'a> CategoryRepository<'a> {
    pub const fn init(db: &'a Lazy<Surreal<Client>>) -> Self {
        CategoryRepository { db: db }
    }

    pub async fn is_exist(&self, id: &String) -> bool {
        let thing_id = thing(id).unwrap();
        let result: Option<CategoryDO> = self.db
            .select(thing_id)
            .await
            .unwrap_or(None);

        match result {
            Some(_) => true,
            None => false,
        }
    }

    pub async fn find_by_id(&self, id: &String) -> surrealdb::Result<Option<CategoryAggregate>> {
        let thing_id = thing(id).unwrap();
        let result: Option<CategoryDO> = self.db
            .select(thing_id)
            .await?;

        let aggregate: Option<CategoryAggregate> = match result {
            Some(value) => Some(CategoryRepoMapper::do_to_aggregate(value)),
            None => None,
        };
        Ok(aggregate)
    }

    pub async fn save(&self, data: CategoryAggregate) -> surrealdb::Result<CategoryAggregate> {
        let category_do = CategoryRepoMapper::aggregate_to_do(data);
        let id = category_do.id.clone();

        let is_new: bool = id.id.to_raw().is_empty();

        // save data
        let result: Option<CategoryDO> = match is_new {
            true => {
                // let db auto generate the id
                self.db
                    .create(tablens::CATEGORY)
                    .content(category_do)
                    .await?
                    .pop()
            }
            false => {
                self.db
                    .update(id)
                    .content(category_do)
                    .await?
            }
        };
        
        let aggregate: CategoryAggregate = CategoryRepoMapper::do_to_aggregate(result.unwrap());

        Ok(aggregate)
    }

    pub async fn delete(&self, id: &String) -> surrealdb::Result<Option<CategoryAggregate>> {
        let thing_id = thing(id).unwrap();
        let result: Option<CategoryDO> = self.db
            .delete(thing_id)
            .await?;

        let aggregate: Option<CategoryAggregate> = match result {
            Some(value) => Some(CategoryRepoMapper::do_to_aggregate(value)),
            None => None,
        };

        Ok(aggregate)
    }
}

