use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::sql::{Datetime, Thing, thing};
use surrealdb::engine::remote::ws::Client;

use crate::common::domain::ID;
use crate::common::repository::{env, relatens};
use crate::resource::domain::ResourceID;
use crate::tag::domain::TagID;

pub static RESOURCE_TAG_RELATION_REPOSITORY: ResourceTagRelationRepository<'_> = ResourceTagRelationRepository::init(&env::DB);
/**
 * Repository */
pub struct ResourceTagRelationRepository<'a> {
    db: &'a Surreal<Client>,
}

impl<'a> ResourceTagRelationRepository<'a> {
    pub const fn init(db: &'a Surreal<Client>) -> Self {
        ResourceTagRelationRepository { db: db }
    }

    async fn delete_relation(&self, tag: &String, resource: &String) -> surrealdb::Result<()> {
        let sql = "DELETE type::table($table) WHERE in == $tag AND out == $resource";

        let _ = self.db
            .query(sql)
            .bind(("table", relatens::TAGGING))
            .bind(("tag", thing(tag).unwrap()))
            .bind(("resource", thing(resource).unwrap()))
            .await?;

        Ok(())
    }

    async fn create_relation(&self, tag: &String, resource: &String) -> surrealdb::Result<()> {
        let sql: String = format!("RELATE $in_id->{}->$out_id", relatens::TAGGING);
        let _ = self.db
            .query(sql)
            .bind(("in_id", thing(tag).unwrap()))
            .bind(("out_id", thing(resource).unwrap()))
            .await?;
        Ok(())
    }

    pub async fn save(&self, target_resource: &ResourceID, new_tags: Vec<TagID>, del_tags: Vec<TagID>) -> surrealdb::Result<()> {
        let resource_id = target_resource.to_string();
        let newiter = new_tags.iter();
        for val in newiter {
            self.create_relation(&val.to_string(), &resource_id).await?
        }

        let deliter = del_tags.iter();
        for val in deliter {
            self.delete_relation(&val.to_string(), &resource_id).await?
        }


        Ok(())
    }
}