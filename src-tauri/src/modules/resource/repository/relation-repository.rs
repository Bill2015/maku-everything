use std::collections::BTreeMap;

use once_cell::sync::Lazy;
use surrealdb::Surreal;
use surrealdb::sql::{thing, Object, Value};
use surrealdb::engine::remote::ws::Client;

use crate::modules::common::domain::DomainModelMapper;
use crate::modules::common::repository::{env, relatens};
use crate::modules::resource::domain::entities::ResourceTaggingEntity;
use crate::modules::resource::domain::valueobj::ResourceTaggingVO;
use crate::modules::resource::domain::ResourceID;

use super::ResourceTaggingDo;

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

    async fn update_relation(&self, tag: &String, resource: &String, tagging: ResourceTaggingDo) -> surrealdb::Result<()> {
        let new_attrval: Value = tagging.attrval.into();
        let sql = format!(r#"
            UPDATE {} 
            SET attrval = {new_attrval}
            WHERE 
                in == $tag_id AND out == $res_id
        "#, relatens::TAGGING);
        
        let _ = self.db
            .query(sql)
            .bind(("tag_id", thing(tag).unwrap()))
            .bind(("res_id", thing(resource).unwrap()))
            .await?;
        Ok(())
    }

    async fn create_relation(&self, tag: &String, resource: &String, tagging: ResourceTaggingDo) -> surrealdb::Result<()> {

        let mut content: BTreeMap<&str, Value> = BTreeMap::new();
        content.insert("added_at", Value::Datetime(tagging.added_at.into()));
        content.insert("tagging_type", Value::Strand(tagging.attrval.to_string().into()));
        content.insert("attrval", tagging.attrval.into());

        let content = Object::from(content);

        let sql = format!(r#"
            RELATE $in_id->{}->$out_id
                CONTENT {content}
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

        let (origin_tags, add_tags, update_tags, del_tags) = tagging.take();
        let adding_tags: Vec<ResourceTaggingVO> = match is_new_resource {
            true => [origin_tags, add_tags].concat(),
            false => add_tags,
        };

        for val in adding_tags {
            let tagging = ResourceTaggingDo::from_domain(val);
            self.create_relation(&tagging.id.to_string(), &resource_id, tagging).await?;
        }

        for val in update_tags {
            let tagging = ResourceTaggingDo::from_domain(val);
            self.update_relation(&tagging.id.to_string(), &resource_id, tagging).await?;
        }

        for val in del_tags {
            self.delete_relation(&val.to_string(), &resource_id).await?
        }

        Ok(())
    }
}