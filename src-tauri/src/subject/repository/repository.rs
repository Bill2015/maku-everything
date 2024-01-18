use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::sql::{Datetime, Thing, thing};
use surrealdb::engine::remote::ws::Client;

use crate::common::infrastructure::IRepoMapper;
use crate::common::repository::{env, tablens, CommonRepository, COMMON_REPOSITORY};
use crate::subject::domain::{SubjectAggregate, SubjectID};
use crate::subject::infrastructure::SubjectRepoMapper;

pub static SUBJECT_REPOSITORY: SubjectRepository<'_> = SubjectRepository::init(&env::DB, &COMMON_REPOSITORY);

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

    pub belong_category: Thing,
}

/**
 * Repository */
pub struct SubjectRepository<'a> {
    db: &'a Lazy<Surreal<Client>>,
    common_repo: &'a CommonRepository<'a>,
}

impl<'a> SubjectRepository<'a> {
    pub const fn init(db: &'a Lazy<Surreal<Client>>, common_repo: &'a CommonRepository) -> Self {
        SubjectRepository {
            db: db,
            common_repo: common_repo,
        }
    }

    async fn return_aggregate_by_id(&self, id: &String) -> surrealdb::Result<Option<SubjectAggregate>> {
        let sql = "SELECT * FROM type::table($table) WHERE id == $id";

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

    pub async fn is_exist(&self, id: &String) -> bool {
        let thing_id = thing(id).unwrap();
        let result: Option<SubjectDO> = self.db
            .select(thing_id)
            .await
            .unwrap_or(None);

        match result {
            Some(value) => true,
            None => false,
        }
    }

    pub async fn find_by_id(&self, id: &String) -> surrealdb::Result<Option<SubjectAggregate>> {
        let result = self.return_aggregate_by_id(id)
            .await?;

        Ok(result)
    }

    pub async fn save(&self, data: SubjectAggregate) -> surrealdb::Result<SubjectAggregate> {
        let belong_category = data.belong_category.clone();

        let subject_do = SubjectRepoMapper::aggregate_to_do(data);
        let id: Thing = subject_do.id.clone();


        let is_new: bool = id.id.to_raw().is_empty();

        // save data
        let result: Option<SubjectDO> = match is_new {
            true => {
                // let db auto generate the id
                self.db
                    .create(tablens::SUBJECT)
                    .content(subject_do)
                    .await?
                    .pop()

            }
            false => {
                self.db
                    .update(id)
                    .content(subject_do)
                    .await?
            }
        };

        let new_id = &result.unwrap().id.to_string();
        // create relation
        if is_new == true {
            let subject_id = SubjectID::from(new_id);
            self.common_repo
                .subject_belong_category(&subject_id, &belong_category)
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

