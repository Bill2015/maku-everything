
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::sql::{Datetime, Thing, thing};
use surrealdb::engine::remote::ws::Client;

use crate::category::domain::CategoryID;
use crate::common::domain::ID;
use crate::common::infrastructure::IRepoMapper;
use crate::common::repository::env;
use crate::common::repository::tablens;
use crate::common::repository::relatens;
use crate::resource;
use crate::resource::domain::ResourceID;
use crate::subject::domain::SubjectID;
use crate::tag::domain::{TagAggregate, TagID};
use crate::tag::infrastructure::TagRepoMapper;

pub static COMMON_REPOSITORY: CommonRepository<'_> = CommonRepository::init(&env::DB);

/**
 * Repository */
 pub struct CommonRepository<'a> {
    db: &'a Lazy<Surreal<Client>>,
}

impl<'a> CommonRepository<'a> {
    pub const fn init(db: &'a Lazy<Surreal<Client>>) -> Self {
        CommonRepository { db: db }
    }

    pub async fn create_belong_relation(&self, in_id: &String, out_id: &String) -> surrealdb::Result<()> {
        let sql: String = format!("RELATE $in_id->{}->$out_id", "belong");
        let _ = self.db
            .query(sql)
            .bind(("in_id", thing(in_id).unwrap()))
            .bind(("out_id", thing(out_id).unwrap()))
            .await?;
        Ok(())
    }

    pub async fn resource_belong_category(&self, resource: &ResourceID, category: &CategoryID) -> surrealdb::Result<()> {
        self.create_belong_relation(
            &resource.to_string(),  // in
            &category.to_string()   // out
        ).await?;
        Ok(())
    }

    pub async fn subject_belong_category(&self, subject: &SubjectID, category: &CategoryID) -> surrealdb::Result<()> {
        self.create_belong_relation(
            &subject.to_string(),  // in
            &category.to_string()   // out
        ).await?;
        Ok(())
    }

    pub async fn tag_belong_category(&self, tag: &TagID, category: &CategoryID) -> surrealdb::Result<()> {
        self.create_belong_relation(
            &tag.to_string(),  // in
            &category.to_string()   // out
        ).await?;
        Ok(())
    }

    pub async fn tag_belong_subject(&self, tag: &TagID, subject: &SubjectID) -> surrealdb::Result<()> {
        self.create_belong_relation(
            &tag.to_string(),  // in
            &subject.to_string()   // out
        ).await?;
        Ok(())
    }
}

