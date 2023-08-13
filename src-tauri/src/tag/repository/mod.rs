use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::sql::{Datetime, Thing};
use surrealdb::engine::remote::ws::Client;

use crate::common::infrastructure::IRepoMapper;
use crate::common::repository::env;
use crate::tag::domain::TagAggregate;
use crate::tag::infrastructure::TagRepoMapper;

/** Database Namespace (aka table name) */
pub const TAG_DB_NAMESPACE: &str = "tag";

pub static TAG_REPOSITORY: TagRepository<'_> = TagRepository::init(&env::DB);

#[derive(Debug, Serialize, Deserialize)]
pub struct TagDO {
   pub id: Option<Thing>,
   pub name: String,
   pub description: String,
   pub belong_category: String, 
   pub auth: bool,
   pub created_at: String,
   pub updated_at: String,
}


/**
 * Repository */
 pub struct TagRepository<'a> {
    db: &'a Surreal<Client>,
}

impl<'a> TagRepository<'a> {
    pub const fn init(db: &'a Surreal<Client>) -> Self {
        TagRepository { db: db }
    }

    pub async fn is_exist(&self, id: String) -> bool {
        let result: Option<TagDO> = self.db
            .select((TAG_DB_NAMESPACE, id))
            .await
            .unwrap_or(None);

        match result {
            Some(value) => true,
            None => false,
        }
    }

    pub async fn find_by_id(&self, id: String) -> surrealdb::Result<Option<TagAggregate>> {
        let response: Option<TagDO> = self.db
            .select((TAG_DB_NAMESPACE, id))
            .await?;

        let aggregate: Option<TagAggregate> = match response {
            Some(value) => Some(TagRepoMapper::do_to_aggregate(value)),
            None => None,
        };
        Ok(aggregate)
    }

    pub async fn save(&self, data: TagAggregate) -> surrealdb::Result<TagAggregate> {
        let mut tag_do = TagRepoMapper::aggregate_to_do(data);
        let id = tag_do.id.clone().unwrap();

        let is_exist: Option<TagDO> = self.db
            .select(id)
            .await?;

        let result: Option<TagDO> = match is_exist {
            Some(value) => {
                self.db
                    .update(value.id.unwrap())
                    .content(tag_do)
                    .await?
            }
            None => {
                tag_do.id = None;
                self.db
                    .create(TAG_DB_NAMESPACE)
                    .content(tag_do)
                    .await?
            }
        };

        let aggregate: TagAggregate = TagRepoMapper::do_to_aggregate(result.unwrap());

        Ok(aggregate)
    }

    pub async fn delete(&self, id: String) -> surrealdb::Result<Option<TagAggregate>> {
        let result: Option<TagDO> = self.db
            .delete((TAG_DB_NAMESPACE, id))
            .await?;

        let aggregate: Option<TagAggregate> = match result {
            Some(value) => Some(TagRepoMapper::do_to_aggregate(value)),
            None => None,
        };

        Ok(aggregate)
    }
}

