#[path ="./query-repository.rs"]
mod query;
pub use query::{TAG_QUERY_REPOSITORY, TagQueryRepository};

use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::sql::{Datetime, Thing, thing};
use surrealdb::engine::remote::ws::Client;

use crate::common::infrastructure::IRepoMapper;
use crate::common::repository::env;
use crate::common::repository::tablens;
use crate::common::repository::relatens;
use crate::tag::domain::TagAggregate;
use crate::tag::infrastructure::TagRepoMapper;

pub static TAG_REPOSITORY: TagRepository<'_> = TagRepository::init(&env::DB);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagDO {
    #[serde(skip_serializing)]
   pub id: Thing,

   pub name: String,
   pub description: String,
   pub auth: bool,
   pub created_at: Datetime,
   pub updated_at: Datetime,

   #[serde(skip_serializing)]
   #[serde(default = "default_ref")]
   pub belong_category: String, 

   #[serde(skip_serializing)]
   #[serde(default = "default_ref")]
   pub belong_subject: String,
}

fn default_ref() -> String {
    "/".to_string()
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

    async fn return_aggregate_by_id(&self, id: &String) -> surrealdb::Result<Option<TagAggregate>> {
        let sql = "SELECT *, type::string((->tag_belong.out)[0]) AS belong_category FROM type::table($table) WHERE id == $id";

        let mut response = self.db
            .query(sql)
            .bind(("table", tablens::TAG))
            .bind(("id", thing(id.as_str()).unwrap()))
            .await?;

        let result: Vec<TagDO> = response
            .take(0)?;

        let item = result
            .first();

        let aggregate = match item {
            Some(value) => Some(TagRepoMapper::do_to_aggregate(value.clone())),
            None => None,
        };

        Ok(aggregate)
    }

    pub async fn is_exist(&self, id: String) -> bool {
        let result: Option<TagDO> = self.db
            .select((tablens::SUBJECT, id))
            .await
            .unwrap_or(None);

        match result {
            Some(value) => true,
            None => false,
        }
    }

    pub async fn find_by_id(&self, id: &String) -> surrealdb::Result<Option<TagAggregate>> {
        let result = self.return_aggregate_by_id(id)
            .await?;

        Ok(result)
    }

    async fn create_belong_category_relation(&self, self_id: &String, category_id: &String) -> surrealdb::Result<()> {
        let sql: String = format!("RELATE $tag->{}->$category", relatens::TAG_BELONG);
        let _ = self.db
            .query(sql)
            .bind(("tag", thing(self_id).unwrap()))
            .bind(("category", thing(category_id).unwrap()))
            .await?;
        Ok(())
    }

    async fn create_belong_subject_relation(&self, self_id: &String, subject_id: &String) -> surrealdb::Result<()> {
        let sql: String = format!("RELATE $tag->{}->$subject", relatens::TAG_BELONG_SUBJECT);
        let _ = self.db
            .query(sql)
            .bind(("tag", thing(self_id).unwrap()))
            .bind(("subject", thing(subject_id).unwrap()))
            .await?;
        Ok(())
    }

    pub async fn save(&self, data: TagAggregate) -> surrealdb::Result<TagAggregate> {
        let tag_do = TagRepoMapper::aggregate_to_do(data);
        let id: Thing = tag_do.id.clone();

        let belong_category = tag_do.belong_category.clone();
        let belong_subject = tag_do.belong_subject.clone();

        let is_new: bool = id.id.to_raw().is_empty();

        // save data
        let result: Option<TagDO> = match is_new {
            true => {
                // let db auto generate the id
                self.db
                    .create(tablens::TAG)
                    .content(tag_do)
                    .await?

            }
            false => {
                self.db
                    .update(id)
                    .content(tag_do)
                    .await?
            }
        };

        let new_id = (&result).as_ref().unwrap().id.to_string();
        // create relation
        if is_new == true {
            self.create_belong_category_relation(&new_id, &belong_category)
                .await?;
            self.create_belong_subject_relation(&new_id, &belong_subject)
                .await?;
        }

        let final_result = self.return_aggregate_by_id(&new_id)
            .await?;

        Ok(final_result.unwrap())
    }

    pub async fn delete(&self, id: String) -> surrealdb::Result<Option<TagAggregate>> {
        let result: Option<TagDO> = self.db
            .delete((tablens::SUBJECT, id))
            .await?;

        let aggregate: Option<TagAggregate> = match result {
            Some(value) => Some(TagRepoMapper::do_to_aggregate(value)),
            None => None,
        };

        Ok(aggregate)
    }
}

