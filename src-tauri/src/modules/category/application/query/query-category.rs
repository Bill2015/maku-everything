use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;

use crate::command_from_dto;
use crate::modules::common::application::IQueryHandler;
use crate::modules::common::infrastructure::QueryBuilder;
use crate::modules::category::domain::CategoryGenericError;
use crate::modules::category::infrastructure::CategoryQueryBuilder;
use crate::modules::category::repository::CategoryQueryRepository;
use crate::modules::category::application::dto::{QueryCategoryDto, CategoryResDto};

#[derive(Deserialize)]
pub struct ListCategoryQuery { 
    pub id: Option<String>,

    pub name: Option<String>,

    pub order_by: Option<String>,
    
    pub limit: Option<i64>,

    pub start: Option<i64>,
}
command_from_dto!(ListCategoryQuery, QueryCategoryDto);

// =====================================
pub struct ListCategoryHandler<'a> {
    category_repo: &'a CategoryQueryRepository<'a>,
}

impl<'a> ListCategoryHandler<'a> {
    pub fn register(category_repo: &'a CategoryQueryRepository) -> Self {
        ListCategoryHandler { category_repo: &category_repo }
    }
}

#[async_trait]
impl IQueryHandler<ListCategoryQuery> for ListCategoryHandler<'_>{
    fn get_name() -> String {
        String::from("Get All Category")
    }

    type Output = Vec<CategoryResDto>;

    async fn query(&self, query: ListCategoryQuery) -> Result<Self::Output, Error> {
        let builder_result = CategoryQueryBuilder::from(query).build()?;

        let result = self.category_repo
            .query(builder_result)
            .await;
    
        match result {
            Ok(value) => Ok(value),
            _ => Err(CategoryGenericError::DBInternalError().into()),
        }
    }
}
