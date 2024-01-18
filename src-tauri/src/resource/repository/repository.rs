use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::sql::{Datetime, Thing, thing};
use surrealdb::engine::remote::ws::Client;

use crate::common::infrastructure::IRepoMapper;
use crate::common::repository::{env, CommonRepository, COMMON_REPOSITORY};
use crate::common::repository::tablens;
use crate::resource::domain::{ResourceAggregate, ResourceID};
use crate::resource::infrastructure::ResourceRepoMapper;

use super::{RESOURCE_TAG_RELATION_REPOSITORY, ResourceTagRelationRepository};

pub static RESOURCE_REPOSITORY: ResourceRepository<'_> = ResourceRepository::init(
    &env::DB,
    &COMMON_REPOSITORY, 
    &RESOURCE_TAG_RELATION_REPOSITORY
);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceFileDo {
    pub uuid: String,
    pub name: String,
    pub path: String,
    pub ext: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceUrlDo {
    pub host: String,
    pub full: String,
}

/**
 * Resource Data Object */
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceDO {
    #[serde(skip_serializing)]
    pub id: Thing,
    pub name: String,
    pub description: String,
    pub file: Option<ResourceFileDo>,
    pub url: Option<ResourceUrlDo>,
    pub auth: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,

    pub belong_category: Thing,

    #[serde(skip_serializing)]
    #[serde(default = "default_vec")]
    pub tags: Vec<Thing>,
}

fn default_vec() -> Vec<Thing> {
    Vec::new()
}
/**
 * Repository */
pub struct ResourceRepository<'a> {
    db: &'a Lazy<Surreal<Client>>,
    common_repo: &'a CommonRepository<'a>,
    tag_relation_repo: &'a ResourceTagRelationRepository<'a> 
}

impl<'a> ResourceRepository<'a> {
    pub const fn init(
        db: &'a Lazy<Surreal<Client>>,
        common_repo: &'a CommonRepository,
        tag_relation_repo: &'a ResourceTagRelationRepository
    ) -> Self {
        ResourceRepository {
            db: db,
            common_repo: common_repo,
            tag_relation_repo: tag_relation_repo,
        }
    }

    async fn return_aggregate_by_id(&self, id: String) -> surrealdb::Result<Option<ResourceAggregate>> {
        let sql = r#"
            SELECT 
                *, 
                <-tagging.in as tags
            FROM type::table($table) 
            WHERE id == $id"#;

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

    pub async fn is_exist(&self, id: &String) -> bool {
        let thing_id = thing(id).unwrap();
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
        dbg!(&data);

        let belong_category = data.belong_category.clone(); 
        let new_tags = data.new_tags.clone();
        let del_tags = data.del_tags.clone();

        let resource_do = ResourceRepoMapper::aggregate_to_do(data);
        let id: Thing = resource_do.id.clone();
        
        let is_new: bool = id.id.to_raw().is_empty();

        // save data
        let result: Option<ResourceDO> = match is_new {
            true => {
                // let db auto generate the id
                self.db
                    .create(tablens::RESOURCE)
                    .content(resource_do)
                    .await?
                    .pop()

            }
            false => {
                self.db
                    .update(id)
                    .content(resource_do)
                    .await?
            }
        };

        let new_id = &result.unwrap().id.to_string();

        // tag adding
        let resource_id = ResourceID::from(new_id);
        self.tag_relation_repo
            .save(&resource_id, new_tags, del_tags)
            .await?;


        // create relation
        if is_new == true {
            self.common_repo
                .resource_belong_category(&resource_id, &belong_category)
                .await?;
        }

        let final_result = self.return_aggregate_by_id(new_id.to_string())
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

