use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Thing, Datetime, thing};

use crate::modules::category::infrastructure::CategoryQueryBuilder;
use crate::modules::common::infrastructure::{QueryBuilder, QueryBuilderResult};
use crate::modules::common::domain::DomainModelMapper;
use crate::modules::common::repository::{env, tablens, CommonRepository, COMMON_REPOSITORY};
use crate::modules::category::domain::{Category, CategoryFactory};

pub static CATEGORY_REPOSITORY: CategoryRepository<'_> = CategoryRepository::init(&env::DB, &COMMON_REPOSITORY);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryMapperRuleItemDO {
    pub text: String,
    pub tag_id: Thing,
}


/**
 * Category Data Object */
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryDO {
    pub id: Thing,
    pub name: String,
    pub description: String,
    pub auth: bool,
    pub root_path: String,
    pub rules: Vec<CategoryMapperRuleItemDO>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}
/**
 * Repository */
pub struct CategoryRepository<'a> {
    db: &'a Lazy<Surreal<Client>>,
    common_repo: &'a CommonRepository<'a>,
}

impl<'a> CategoryRepository<'a> {
    pub const fn init(db: &'a Lazy<Surreal<Client>>, common_repo: &'a CommonRepository) -> Self {
        CategoryRepository {
            db: db,
            common_repo: common_repo, 
        }
    }

    pub async fn get_by(&self, builder_result: QueryBuilderResult) -> surrealdb::Result<Vec<Category>> {
        let sql = format!(r#"
            SELECT 
                *
            FROM type::table($table) WHERE {}"#, 
        builder_result.to_string());

        let result: Vec<Category> = self.db
            .query(sql)
            .bind(("table", tablens::CATEGORY))
            .await?
            .take::<Vec<CategoryDO>>(0)?
            .into_iter()
            .map(|val| Self::model_to_entity(val))
            .collect();

        Ok(result) 
    }

    pub async fn is_exist(&self, id: &String) -> bool {
        let thing_id = thing(id).unwrap();
        let result: Option<CategoryDO> = self.db
            .select(thing_id)
            .await
            .unwrap_or(None);

        match result {
            Some(_) => true,
            None => false,
        }
    }

    pub async fn is_duplicate_name(&self, name: &String) -> surrealdb::Result<bool> {
        let buildres = CategoryQueryBuilder::new()
            .set_name(name)
            .build()
            .unwrap();

        let result = self.common_repo.is_duplicated(tablens::CATEGORY, buildres)
            .await?;

        Ok(result)
    }

    pub async fn find_by_id(&self, id: &String) -> surrealdb::Result<Option<Category>> {
        let thing_id = thing(id).unwrap();
        let result: Option<CategoryDO> = self.db
            .select(thing_id)
            .await?;

        let aggregate: Option<Category> = match result {
            Some(value) => Some(Self::model_to_entity(value)),
            None => None,
        };
        Ok(aggregate)
    }

    pub async fn save(&self, data: Category) -> surrealdb::Result<Category> {
        let category_do: CategoryDO = Self::entity_to_model(data);

        let id = category_do.id.clone();

        let is_new: bool = !self.is_exist(&id.to_string()).await;

        // save data
        let result: Option<CategoryDO> = match is_new {
            true => {
                // let db auto generate the id
                self.db
                    .create(tablens::CATEGORY)
                    .content(category_do)
                    .await?
                    .pop()
            }
            false => {
                self.db
                    .update(id)
                    .content(category_do)
                    .await?
            }
        };
        
        Ok(Self::model_to_entity(result.unwrap()))
    }

    pub async fn delete(&self, id: &String) -> surrealdb::Result<Option<Category>> {
        let thing_id = thing(id).unwrap();
        let result: Option<CategoryDO> = self.db
            .delete(thing_id)
            .await?;

        let aggregate: Option<Category> = match result {
            Some(value) => Some(Self::model_to_entity(value)),
            None => None,
        };

        Ok(aggregate)
    }

    fn entity_to_model(entity: Category) -> CategoryDO {
        CategoryDO::from_domain(entity.to_properties())
    }

    fn model_to_entity(model: CategoryDO) -> Category {
        CategoryFactory::reconstitute(model.to_domain())
    }
}

