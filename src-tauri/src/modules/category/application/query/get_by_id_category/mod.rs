use anyhow::Error;
use async_trait::async_trait;

use crate::modules::common::application::IQueryHandler;
use crate::modules::category::repository::CategoryQueryRepository;
use crate::modules::category::application::dto::CategoryResDto;
use crate::modules::category::domain::CategoryGenericError;

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

    type Output = Option<CategoryResDto>;

    async fn query(&self, query: GetByIdCategoryQuery) -> Result<Self::Output, Error> {
        let GetByIdCategoryQuery { id } = query;

        let result = self.category_repo
            .get_by_id(&id)
            .await;
    
        match result {
            Ok(value) => Ok(value),
            _ => Err(CategoryGenericError::DBInternalError().into()),
        }
    }
}
