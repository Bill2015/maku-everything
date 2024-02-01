use once_cell::sync::Lazy;
use surrealdb::Surreal;
use surrealdb::sql::thing;
use surrealdb::engine::remote::ws::Client;

use crate::modules::common::repository::{env, relatens};
use crate::modules::resource::domain::entities::ResourceTaggingEntity;
use crate::modules::resource::domain::ResourceID;

pub static RESOURCE_TAG_RELATION_REPOSITORY: ResourceTagRelationRepository<'_> = ResourceTagRelationRepository::init(&env::DB);
/**
 * Repository */
pub struct ResourceTagRelationRepository<'a> {
    db: &'a Lazy<Surreal<Client>>,
}

impl<'a> ResourceTagRelationRepository<'a> {
    pub const fn init(db: &'a Lazy<Surreal<Client>>) -> Self {
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
        let sql: String = format!(r#"
            RELATE $in_id->{}->$out_id
                SET added_at = time::now()
        "#, relatens::TAGGING);
        let _ = self.db
            .query(sql)
            .bind(("in_id", thing(tag).unwrap()))
            .bind(("out_id", thing(resource).unwrap()))
            .await?;
        Ok(())
    }

    pub async fn save(&self, target_resource: &ResourceID, is_new_resource: bool, tagging: ResourceTaggingEntity) -> surrealdb::Result<()> {
        let resource_id = target_resource.to_string();

        if is_new_resource {
            for val in tagging.vals() {
                self.create_relation(&val.id.to_string(), &resource_id).await?
            }
        }

        for val in tagging.get_add_tags() {
            self.create_relation(&val.id.to_string(), &resource_id).await?
        }

        for val in  tagging.get_del_tags() {
            self.delete_relation(&val.id.to_string(), &resource_id).await?
        }


        Ok(())
    }
}