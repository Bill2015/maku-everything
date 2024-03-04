
use once_cell::sync::Lazy;
use surrealdb::Surreal;
use surrealdb::sql::thing;
use surrealdb::engine::remote::ws::Client;

use crate::modules::category::domain::CategoryID;
use crate::modules::common::infrastructure::QueryBuilderResult;
use crate::modules::common::repository::env;
use crate::modules::resource::domain::ResourceID;
use crate::modules::subject::domain::SubjectID;
use crate::modules::tag::domain::TagID;

use super::CountDO;

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

    pub async fn is_duplicated(&self, ns: &str, builder_result: QueryBuilderResult) -> surrealdb::Result<bool> {
        let sql = format!(r#"
            SELECT 
                COUNT()
            FROM type::table($table) 
            WHERE {}"#, 
            builder_result.to_string());
        
        let result = self.db
            .query(sql)
            .bind(("table", ns))
            .await?
            .take::<Vec<CountDO>>(0)?
            .pop();

        Ok(result.is_some() && (result.unwrap().count > 0))
    }
}

