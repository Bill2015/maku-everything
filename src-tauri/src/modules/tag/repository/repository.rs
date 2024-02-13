use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::sql::{Datetime, Thing, thing};
use surrealdb::engine::remote::ws::Client;

use crate::modules::common::domain::DomainModelMapper;
use crate::modules::common::infrastructure::QueryBuilderResult;
use crate::modules::common::repository::env;
use crate::modules::common::repository::tablens;
use crate::modules::common::repository::{CommonRepository, COMMON_REPOSITORY};
use crate::modules::tag::domain::TagFactory;
use crate::tag::domain::{Tag, TagID};

pub static TAG_REPOSITORY: TagRepository<'_> = TagRepository::init(&env::DB, &COMMON_REPOSITORY);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagDO {
   pub id: Thing,

   pub name: String,
   pub description: String,
   pub auth: bool,
   pub created_at: Datetime,
   pub updated_at: Datetime,

   pub belong_category: Thing, 

   pub belong_subject: Thing,
}

/**
 * Repository */
 pub struct TagRepository<'a> {
    db: &'a Lazy<Surreal<Client>>,
    common_repo: &'a CommonRepository<'a>,
}

impl<'a> TagRepository<'a> {
    pub const fn init(db: &'a Lazy<Surreal<Client>>, common_repo: &'a CommonRepository<'a>) -> Self {
        TagRepository {
            db: db,
            common_repo: common_repo,
        }
    }

    pub async fn get_by(&self, builder_result: QueryBuilderResult) -> surrealdb::Result<Vec<Tag>> {
        let sql = format!(r#"
            SELECT 
                *
            FROM type::table($table) WHERE {}"#, 
            builder_result.to_string());

        let result: Vec<Tag> = self.db
            .query(sql)
            .bind(("table", tablens::TAG))
            .await?
            .take::<Vec<TagDO>>(0)?
            .into_iter()
            .map(|val| Self::model_to_entity(val))
            .collect();

        Ok(result) 
    }

    async fn return_aggregate_by_id(&self, id: &String) -> surrealdb::Result<Option<Tag>> {
        let sql = "SELECT * FROM type::table($table) WHERE id == $id";

        let mut response = self.db
            .query(sql)
            .bind(("table", tablens::TAG))
            .bind(("id", thing(id.as_str()).unwrap()))
            .await?;

        let result: Option<Tag> = response
            .take::<Vec<TagDO>>(0)?
            .pop()
            .map(|val| Self::model_to_entity(val));

        Ok(result)
    }

    pub async fn is_exist(&self, id: &String) -> bool {
        let thing_id = thing(id).unwrap();
        let result: Option<TagDO> = self.db
            .select(thing_id)
            .await
            .unwrap_or(None);

        match result {
            Some(_) => true,
            None => false,
        }
    }

    pub async fn find_by_id(&self, id: &String) -> surrealdb::Result<Option<Tag>> {
        let result = self.return_aggregate_by_id(id)
            .await?;

        Ok(result)
    }

    pub async fn save(&self, data: Tag) -> surrealdb::Result<Tag> {
        let belong_category = data.get_belong_category().clone();
        let belong_subject = data.get_belong_subject().clone();

        let tag_do: TagDO = Self::entity_to_model(data);
        let id: Thing = tag_do.id.clone();

        let is_new: bool = !self.is_exist(&id.to_string()).await;

        // save data
        let result: Option<TagDO> = match is_new {
            true => {
                // let db auto generate the id
                self.db
                    .create(tablens::TAG)
                    .content(tag_do)
                    .await?
                    .pop()
            }
            false => {
                self.db
                    .update(id)
                    .content(tag_do)
                    .await?
            }
        };

        let new_id = &result.unwrap().id.to_string();
        // create relation
        if is_new == true {
            let tag_id = TagID::from(new_id);
            self.common_repo
                .tag_belong_category(&tag_id, &belong_category)
                .await?;
    
            self.common_repo
                .tag_belong_subject(&tag_id, &belong_subject)
                .await?;
        }

        let final_result = self.return_aggregate_by_id(&new_id)
            .await?;

        Ok(final_result.unwrap())
    }

    pub async fn delete(&self, id: String) -> surrealdb::Result<Option<Tag>> {
        let result: Option<TagDO> = self.db
            .delete((tablens::SUBJECT, id))
            .await?;

        let aggregate: Option<Tag> = match result {
            Some(value) => Some(Self::model_to_entity(value)),
            None => None,
        };

        Ok(aggregate)
    }

    fn entity_to_model(entity: Tag) -> TagDO {
        TagDO::from_domain(entity.to_properties())
    }

    fn model_to_entity(model: TagDO) -> Tag {
        TagFactory::reconstitute(model.to_domain())
    }
}

