#[path ="./query-repository.rs"]
mod query;
pub use query::{CATEGORY_QUERY_REPOSITORY, CategoryQueryRepository};

use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;

use crate::common::infrastructure::IRepoMapper;
use crate::common::repository::env;
use crate::category::domain::CategoryAggregate;
use crate::category::infrastructure::CategoryRepoMapper;

/** Database Namespace (aka table name) */
pub const CATEGORY_DB_NAMESPACE: &str = "category";

pub static CATEGORY_REPOSITORY: CategoryRepository<'_> = CategoryRepository::init(&env::DB);

/**
 * Category Data Object */
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryDO {
    pub id: Option<Thing>,
    pub title: String,
    pub description: String,
    pub auth: bool,
    pub created_at: String,
    pub updated_at: String,
}
/**
 * Repository */
pub struct CategoryRepository<'a> {
    db: &'a Surreal<Client>,
}

impl<'a> CategoryRepository<'a> {
    pub const fn init(db: &'a Surreal<Client>) -> Self {
        CategoryRepository { db: db }
    }

    pub async fn is_exist(&self, id: &String) -> bool {
        let result: Option<CategoryDO> = self.db
            .select((CATEGORY_DB_NAMESPACE, id))
            .await
            .unwrap_or(None);

        match result {
            Some(value) => true,
            None => false,
        }
    }

    pub async fn find_by_id(&self, id: &String) -> surrealdb::Result<Option<CategoryAggregate>> {
        let result: Option<CategoryDO> = self.db
            .select((CATEGORY_DB_NAMESPACE, id))
            .await?;

        let aggregate: Option<CategoryAggregate> = match result {
            Some(value) => Some(CategoryRepoMapper::do_to_aggregate(value)),
            None => None,
        };
        Ok(aggregate)
    }

    pub async fn save(&self, data: CategoryAggregate) -> surrealdb::Result<CategoryAggregate> {
        let mut category_do = CategoryRepoMapper::aggregate_to_do(data);
        let id = category_do.id.clone().unwrap();

        let is_exist: Option<CategoryDO> = self.db
            .select(id)
            .await?;

        let result: Option<CategoryDO> = match is_exist {
            Some(value) => {
                self.db
                    .update(value.id.unwrap())
                    .content(category_do)
                    .await?
            }
            None => {
                // let db auto generate the id
                category_do.id = None;
                self.db
                    .create(CATEGORY_DB_NAMESPACE)
                    .content(category_do)
                    .await?
            }
        };

        let aggregate: CategoryAggregate = CategoryRepoMapper::do_to_aggregate(result.unwrap());

        Ok(aggregate)
    }

    pub async fn delete(&self, id: String) -> surrealdb::Result<Option<CategoryAggregate>> {
        let result: Option<CategoryDO> = self.db
            .delete((CATEGORY_DB_NAMESPACE, id))
            .await?;

        let aggregate: Option<CategoryAggregate> = match result {
            Some(value) => Some(CategoryRepoMapper::do_to_aggregate(value)),
            None => None,
        };

        Ok(aggregate)
    }
}

