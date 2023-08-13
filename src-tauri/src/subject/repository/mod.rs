use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::sql::{Datetime, Thing};
use surrealdb::engine::remote::ws::Client;

use crate::common::infrastructure::IRepoMapper;
use crate::common::repository::env;
use crate::subject::domain::SubjectAggregate;
use crate::subject::infrastructure::SubjectRepoMapper;

/** Database Namespace (aka table name) */
pub const SUBJECT_DB_NAMESPACE: &str = "subject";

pub static SUBJECT_REPOSITORY: SubjectRepository<'_> = SubjectRepository::init(&env::DB);

/**
 * Subject Data Object */
#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectDO {
    pub id: Option<Thing>,
    pub name: String,
    pub description: String,
    pub belong_category: Thing,
    pub auth: bool,
    pub created_at: String,
    pub updated_at: String,
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

    pub async fn is_exist(&self, id: String) -> bool {
        let result: Option<SubjectDO> = self.db
            .select((SUBJECT_DB_NAMESPACE, id))
            .await
            .unwrap_or(None);

        match result {
            Some(value) => true,
            None => false,
        }
    }

    pub async fn find_by_id(&self, id: String) -> surrealdb::Result<Option<SubjectAggregate>> {
        let response: Option<SubjectDO> = self.db
            .select((SUBJECT_DB_NAMESPACE, id))
            .await?;

        let aggregate: Option<SubjectAggregate> = match response {
            Some(value) => Some(SubjectRepoMapper::do_to_aggregate(value)),
            None => None,
        };
        Ok(aggregate)
    }

    pub async fn save(&self, data: SubjectAggregate) -> surrealdb::Result<SubjectAggregate> {
        let mut subject_do = SubjectRepoMapper::aggregate_to_do(data);
        let id = subject_do.id.clone().unwrap();

        let is_exist: Option<SubjectDO> = self.db
            .select(id)
            .await?;

        let result: Option<SubjectDO> = match is_exist {
            Some(value) => {
                self.db
                    .update(value.id.unwrap())
                    .content(subject_do)
                    .await?
            }
            None => {
                subject_do.id = None;
                self.db
                    .create(SUBJECT_DB_NAMESPACE)
                    .content(subject_do)
                    .await?
            }
        };

        let aggregate: SubjectAggregate = SubjectRepoMapper::do_to_aggregate(result.unwrap());

        Ok(aggregate)
    }

    pub async fn delete(&self, id: String) -> surrealdb::Result<Option<SubjectAggregate>> {
        let result: Option<SubjectDO> = self.db
            .delete((SUBJECT_DB_NAMESPACE, id))
            .await?;

        let aggregate: Option<SubjectAggregate> = match result {
            Some(value) => Some(SubjectRepoMapper::do_to_aggregate(value)),
            None => None,
        };

        Ok(aggregate)
    }
}

