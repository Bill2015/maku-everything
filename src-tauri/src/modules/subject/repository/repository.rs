use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::sql::{Datetime, Thing, thing};
use surrealdb::engine::remote::ws::Client;

use crate::modules::common::domain::DomainModelMapper;
use crate::modules::common::infrastructure::QueryBuilderResult;
use crate::modules::common::repository::{env, tablens, CommonRepository, COMMON_REPOSITORY};
use crate::modules::subject::domain::{Subject, SubjectFactory, SubjectID};

pub static SUBJECT_REPOSITORY: SubjectRepository<'_> = SubjectRepository::init(&env::DB, &COMMON_REPOSITORY);

/**
 * Subject Data Object */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubjectDO {
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

    pub async fn get_by(&self, builder_result: QueryBuilderResult) -> surrealdb::Result<Vec<Subject>> {
        let sql = format!(r#"
            SELECT 
                *
            FROM type::table($table) WHERE {}"#, 
            builder_result.to_string());

        let result: Vec<Subject> = self.db
            .query(sql)
            .bind(("table", tablens::SUBJECT))
            .await?
            .take::<Vec<SubjectDO>>(0)?
            .into_iter()
            .map(|val| Self::model_to_entity(val))
            .collect();

        Ok(result) 
    }

    async fn return_aggregate_by_id(&self, id: &String) -> surrealdb::Result<Option<Subject>> {
        let sql = "SELECT * FROM type::table($table) WHERE id == $id";

        let mut response = self.db
            .query(sql)
            .bind(("table", tablens::SUBJECT))
            .bind(("id", thing(id.as_str()).unwrap()))
            .await?;

        let result: Option<Subject> = response
            .take::<Vec<SubjectDO>>(0)?
            .pop()
            .map(|val| Self::model_to_entity(val));

        Ok(result)
    }

    pub async fn is_exist(&self, id: &String) -> bool {
        let thing_id = thing(id).unwrap();
        let result: Option<SubjectDO> = self.db
            .select(thing_id)
            .await
            .unwrap_or(None);

        match result {
            Some(_) => true,
            None => false,
        }
    }

    pub async fn find_by_id(&self, id: &String) -> surrealdb::Result<Option<Subject>> {
        let result = self.return_aggregate_by_id(id)
            .await?;

        Ok(result)
    }

    pub async fn save(&self, data: Subject) -> surrealdb::Result<Subject> {
        let belong_category = data.get_belong_category().clone();

        let subject_do: SubjectDO = Self::entity_to_model(data);
        let id: Thing = subject_do.id.clone();

        let is_new: bool = !self.is_exist(&id.to_string()).await;

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

    pub async fn delete(&self, id: String) -> surrealdb::Result<Option<Subject>> {
        let result: Option<SubjectDO> = self.db
            .delete((tablens::SUBJECT, id))
            .await?;

        let aggregate: Option<Subject> = match result {
            Some(value) => Some(Self::model_to_entity(value)),
            None => None,
        };

        Ok(aggregate)
    }

    fn entity_to_model(entity: Subject) -> SubjectDO {
        SubjectDO::from_domain(entity.to_properties())
    }

    fn model_to_entity(model: SubjectDO) -> Subject {
        SubjectFactory::reconstitute(model.to_domain())
    }
}

