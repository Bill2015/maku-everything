use once_cell::sync::Lazy;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::thing;

use crate::modules::common::repository::{env, tablens};
use crate::modules::category::application::dto::CategoryResDto;

pub static CATEGORY_QUERY_REPOSITORY: CategoryQueryRepository<'_> = CategoryQueryRepository::init(&env::DB);

pub struct CategoryQueryRepository<'a> {
    db: &'a Lazy<Surreal<Client>>,
}
impl<'a> CategoryQueryRepository<'a> {
    const RESOURCE_NUM_FIELD: &str = "array::len(<-belong<-resource) AS resource_num";


    pub const fn init(db: &'a Lazy<Surreal<Client>>) -> Self {
        CategoryQueryRepository { db: db }
    }

    pub async fn get_all(&self) -> surrealdb::Result<Vec<CategoryResDto>> {
        let sql = format!("SELECT *, {} FROM type::table($table)", Self::RESOURCE_NUM_FIELD);

        let mut response = self.db
            .query(sql)
            .bind(("table", &tablens::CATEGORY))
            .await?;

        let result: Vec<CategoryResDto> = response
            .take(0)
            .unwrap();

        Ok(result)
    }

    pub async fn get_by_id(&self, id: &String) -> surrealdb::Result<Option<CategoryResDto>> {
        let sql = format!("SELECT *, {} FROM type::table($table) WHERE id == $id", Self::RESOURCE_NUM_FIELD);

        let mut response = self.db
            .query(sql)
            .bind(("id", thing(id.as_str())))
            .bind(("table", &tablens::CATEGORY))
            .await?;

        let result: Option<CategoryResDto> = response
            .take(0)
            .unwrap_or(None);

        Ok(result) 
    }
}