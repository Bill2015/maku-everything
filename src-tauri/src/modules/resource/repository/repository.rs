use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::sql::{Datetime, Thing, thing};
use surrealdb::engine::remote::ws::Client;

use crate::modules::common::domain::DomainModelMapper;
use crate::modules::common::infrastructure::QueryBuilderResult;
use crate::modules::common::repository::{env, CommonRepository, COMMON_REPOSITORY};
use crate::modules::common::repository::tablens;
use crate::modules::resource::domain::{Resource, ResourceFactory, ResourceID};

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceTaggingDo {
    #[serde(alias = "in")]
    pub id: Thing,

    pub added_at: Datetime,
}

/**
 * Resource Data Object */
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceDO {
    pub id: Thing,
    pub name: String,
    pub description: String,
    pub root_path: String,
    pub file: Option<ResourceFileDo>,
    pub url: Option<ResourceUrlDo>,
    pub auth: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,

    pub belong_category: Thing,

    #[serde(skip_serializing)]
    #[serde(default = "Vec::new")]
    pub tags: Vec<ResourceTaggingDo>,
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

    pub async fn get_by(&self, builder_result: QueryBuilderResult) -> surrealdb::Result<Vec<Resource>> {
        let sql = format!(r#"
            SELECT 
                *,
                belong_category.root_path as root_path,
                <-tagging.* AS tags
            OMIT tags.id, tags.out
            FROM type::table($table) WHERE {}"#, 
            builder_result.to_string());

        let result: Vec<Resource> = self.db
            .query(sql)
            .bind(("table", tablens::RESOURCE))
            .await?
            .take::<Vec<ResourceDO>>(0)?
            .into_iter()
            .map(|val| Self::model_to_entity(val))
            .collect();

        Ok(result) 
    }

    async fn return_aggregate_by_id(&self, id: String) -> surrealdb::Result<Option<Resource>> {
        let sql = r#"
            SELECT 
                *,
                belong_category.root_path as root_path,
                <-tagging.* AS tags
            OMIT tags.id, tags.out
            FROM type::table($table) 
            WHERE id == $id"#;

        let mut response = self.db
            .query(sql)
            .bind(("table", tablens::RESOURCE))
            .bind(("id", thing(id.as_str()).unwrap()))
            .await?;

        let result: Option<Resource> = response
            .take::<Vec<ResourceDO>>(0)?
            .pop()
            .map(|val| Self::model_to_entity(val));

        Ok(result)
    }

    pub async fn is_exist(&self, id: &String) -> bool {
        let thing_id = thing(id).unwrap();
        let result: Option<ResourceDO> = self.db
            .select(thing_id)
            .await
            .unwrap_or(None);

        match result {
            Some(_) => true,
            None => false,
        }
    }

    pub async fn find_by_id(&self, id: String) -> surrealdb::Result<Option<Resource>> {
        let result = self.return_aggregate_by_id(id)
            .await?;

        Ok(result)
    }

    pub async fn save(&self, data: Resource) -> surrealdb::Result<Resource> {
        let belong_category = data.get_belong_category().clone(); 
        let tagging = data.get_tagging().clone();

        let resource_do: ResourceDO = Self::entity_to_model(data);
        let id: Thing = resource_do.id.clone();
        
        let is_new: bool = !self.is_exist(&id.to_string()).await;

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
            .save(&resource_id, is_new, tagging)
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

    pub async fn delete(&self, id: String) -> surrealdb::Result<Option<Resource>> {
        let thing_id = thing(id.as_str()).unwrap();
        let result: Option<ResourceDO> = self.db
            .delete(thing_id)
            .await?;

        let aggregate: Option<Resource> = match result {
            Some(value) => Some(Self::model_to_entity(value)),
            None => None,
        };

        Ok(aggregate)
    }

    fn entity_to_model(entity: Resource) -> ResourceDO {
        ResourceDO::from_domain(entity.to_properties())
    }

    fn model_to_entity(model: ResourceDO) -> Resource {
        ResourceFactory::reconstitute(model.to_domain())
    }
}
