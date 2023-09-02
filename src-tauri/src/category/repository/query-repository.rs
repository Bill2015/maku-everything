use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Thing, thing};

use crate::common::repository::{env, tablens};
use crate::category::application::dto::CategoryResDto;

pub static CATEGORY_QUERY_REPOSITORY: CategoryQueryRepository<'_> = CategoryQueryRepository::init(&env::DB);

pub struct CategoryQueryRepository<'a> {
    db: &'a Lazy<Surreal<Client>>,
}
impl<'a> CategoryQueryRepository<'a> {
    pub const fn init(db: &'a Lazy<Surreal<Client>>) -> Self {
        CategoryQueryRepository { db: db }
    }

    pub async fn get_all(&self) -> surrealdb::Result<Vec<CategoryResDto>> {
        let mut response = self.db
            .query("SELECT *, array::len(<-belong<-resource) AS resource_num FROM type::table($table)")
            .bind(("table", &tablens::CATEGORY))
            .await?;

        let result: Vec<CategoryResDto> = response
            .take(0)
            .unwrap();

        Ok(result)
    }

    pub async fn get_by_id(&self, id: &String) -> surrealdb::Result<Option<CategoryResDto>> {
        let mut response = self.db
            .query("SELECT *, array::len(<-belong<-resource) AS resource_num FROM category WHERE id == $id")
            .bind(("id", thing(id.as_str())))
            .await?;

        let result: Option<CategoryResDto> = response
            .take(0)
            .unwrap_or(None);

        Ok(result) 
    }
}