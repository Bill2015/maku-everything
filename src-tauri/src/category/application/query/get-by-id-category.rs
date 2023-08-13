use async_trait::async_trait;

use crate::common::application::IQueryHandler;
use crate::category::repository::CategoryQueryRepository;
use crate::category::application::dto::{CategoryResDto, CategoryError};

pub struct GetByIdCategoryQuery { 
    pub id: String,
}

// =====================================
pub struct GetByIdCategoryHandler<'a> {
    category_repo: &'a CategoryQueryRepository<'a>,
}

impl<'a> GetByIdCategoryHandler<'a> {
    pub fn register(category_repo: &'a CategoryQueryRepository) -> Self {
        GetByIdCategoryHandler { category_repo: &category_repo }
    }
}

#[async_trait]
impl IQueryHandler<GetByIdCategoryQuery> for GetByIdCategoryHandler<'_>{
    fn get_name() -> String {
        String::from("Get All Category")
    }

    type Output = Result<Option<CategoryResDto>, CategoryError>;

    async fn query(&self, query: GetByIdCategoryQuery) -> Self::Output {
        let GetByIdCategoryQuery { id } = query;

        let result = self.category_repo
            .get_by_id(&id)
            .await;
    
        match result {
            Ok(value) => Ok(value),
            _ => Err(CategoryError::FindById(id)),
        }
    }
}
