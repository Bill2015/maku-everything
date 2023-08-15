#[path ="./query-repository.rs"]
mod query;
pub use query::{SUBJECT_QUERY_REPOSITORY, SubjectQueryRepository};

use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::sql::{Datetime, Thing, thing};
use surrealdb::engine::remote::ws::Client;

use crate::common::infrastructure::IRepoMapper;
use crate::common::repository::{env, relatens, tablens};
use crate::subject::domain::SubjectAggregate;
use crate::subject::infrastructure::SubjectRepoMapper;

pub static SUBJECT_REPOSITORY: SubjectRepository<'_> = SubjectRepository::init(&env::DB);

/**
 * Subject Data Object */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubjectDO {
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
}

fn default_ref() -> String {
    "/".to_string()
}
/**
 * Repository */
pub struct SubjectRepository<'a> {
    db: &'a Surreal<Client>,
}

impl<'a> SubjectRepository<'a> {
    pub const fn init(db: &'a Surreal<Client>) -> Self {
        SubjectRepository { db: db }
    }

    async fn return_aggregate_by_id(&self, id: &String) -> surrealdb::Result<Option<SubjectAggregate>> {
        let sql = "SELECT *, type::string((->subject_belong.out)[0]) AS belong_category FROM type::table($table) WHERE id == $id";

        let mut response = self.db
            .query(sql)
            .bind(("table", tablens::SUBJECT))
            .bind(("id", thing(id.as_str()).unwrap()))
            .await?;

        let result: Vec<SubjectDO> = response
            .take(0)?;

        let item = result
            .first();


        let aggregate = match item {
            Some(value) => Some(SubjectRepoMapper::do_to_aggregate(value.clone())),
            None => None,
        };

        Ok(aggregate)
    }

    pub async fn is_exist(&self, id: String) -> bool {
        let result: Option<SubjectDO> = self.db
            .select((tablens::SUBJECT, id))
            .await
            .unwrap_or(None);

        match result {
            Some(value) => true,
            None => false,
        }
    }

    async fn create_belong_category_relation(&self, self_id: &String, category_id: &String) -> surrealdb::Result<()> {
        let sql: String = format!("RELATE $subject->{}->$category", relatens::SUBJECT_BELONG);
        let _ = self.db
            .query(sql)
            .bind(("subject", thing(self_id).unwrap()))
            .bind(("category", thing(category_id).unwrap()))
            .await?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: &String) -> surrealdb::Result<Option<SubjectAggregate>> {
        let result = self.return_aggregate_by_id(id)
            .await?;

        Ok(result)
    }

    pub async fn save(&self, data: SubjectAggregate) -> surrealdb::Result<SubjectAggregate> {
        let subject_do = SubjectRepoMapper::aggregate_to_do(data);
        let id: Thing = subject_do.id.clone();

        let belong_category = subject_do.belong_category.clone();

        let is_new: bool = id.id.to_raw().is_empty();

        // save data
        let result: Option<SubjectDO> = match is_new {
            true => {
                // let db auto generate the id
                self.db
                    .create(tablens::SUBJECT)
                    .content(subject_do)
                    .await?

            }
            false => {
                self.db
                    .update(id)
                    .content(subject_do)
                    .await?
            }
        };

        let new_id = (&result).as_ref().unwrap().id.to_string();
        // create relation
        if is_new == true {
            self.create_belong_category_relation(&new_id, &belong_category)
                .await?;
        }

        let final_result = self.return_aggregate_by_id(&new_id)
            .await?;

        Ok(final_result.unwrap())
    }

    pub async fn delete(&self, id: String) -> surrealdb::Result<Option<SubjectAggregate>> {
        let result: Option<SubjectDO> = self.db
            .delete((tablens::SUBJECT, id))
            .await?;

        let aggregate: Option<SubjectAggregate> = match result {
            Some(value) => Some(SubjectRepoMapper::do_to_aggregate(value)),
            None => None,
        };

        Ok(aggregate)
    }
}

