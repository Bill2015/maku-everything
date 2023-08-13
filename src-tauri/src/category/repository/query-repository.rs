use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Thing, thing};

use crate::common::repository::env;
use crate::category::application::dto::CategoryResDto;

use super::CATEGORY_DB_NAMESPACE;

pub static CATEGORY_QUERY_REPOSITORY: CategoryQueryRepository<'_> = CategoryQueryRepository::init(&env::DB);

pub struct CategoryQueryRepository<'a> {
    db: &'a Surreal<Client>,
}
impl<'a> CategoryQueryRepository<'a> {
    pub const fn init(db: &'a Surreal<Client>) -> Self {
        CategoryQueryRepository { db: db }
    }

    pub async fn get_all(&self) -> surrealdb::Result<Vec<CategoryResDto>> {
        let mut response = self.db
            .query("SELECT *, array::len(<-resource_belong.in) AS resource_num FROM type::table($table)")
            .bind(("table", &CATEGORY_DB_NAMESPACE))
            .await?;

        let result: Vec<CategoryResDto> = response
            .take(0)
            .unwrap();

        Ok(result)
    }

    pub async fn get_by_id(&self, id: &String) -> surrealdb::Result<Option<CategoryResDto>> {
        let mut response = self.db
            .query("SELECT *, array::len(<-resource_belong.in) AS resource_num FROM category WHERE id == $id")
            .bind(("id", thing(id.as_str())))
            .await?;

        let result: Option<CategoryResDto> = response
            .take(0)
            .unwrap_or(None);

        Ok(result) 
    }
}