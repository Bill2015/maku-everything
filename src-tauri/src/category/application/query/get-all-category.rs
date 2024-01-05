use std::fmt;
use async_trait::async_trait;

use crate::common::application::IQueryHandler;
use crate::category::repository::CategoryQueryRepository;
use crate::category::application::dto::CategoryResDto;
use crate::category::domain::{CategoryError, CategoryGenericError};

pub struct GetAllCategoryQuery { }

// =====================================
pub struct GetAllCategoryHandler<'a> {
    category_repo: &'a CategoryQueryRepository<'a>,
}

impl<'a> GetAllCategoryHandler<'a> {
    pub fn register(category_repo: &'a CategoryQueryRepository) -> Self {
        GetAllCategoryHandler { category_repo: &category_repo }
    }
}

#[async_trait]
impl IQueryHandler<GetAllCategoryQuery> for GetAllCategoryHandler<'_>{
    fn get_name() -> String {
        String::from("Get All Category")
    }

    type Output = Result<Vec<CategoryResDto>, CategoryError>;

    async fn query(&self, query: GetAllCategoryQuery) -> Self::Output {
        let result = self.category_repo
            .get_all()
            .await;
    
        match result {
            Ok(value) => Ok(value),
            _ => Err(CategoryError::GetAll(CategoryGenericError::Unknown{ message: String::from("uncertain error") })),
        }
    }
}
