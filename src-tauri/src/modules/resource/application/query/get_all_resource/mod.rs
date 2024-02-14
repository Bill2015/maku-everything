use anyhow::Error;
use async_trait::async_trait;

use crate::modules::common::application::IQueryHandler;
use crate::modules::resource::domain::ResourceGenericError;
use crate::modules::resource::repository::ResourceQueryRepository;
use crate::modules::resource::application::dto::ResourceResDto;

pub struct GetAllResourceQuery { }

// =====================================
pub struct GetAllResourceHandler<'a> {
    category_repo: &'a ResourceQueryRepository<'a>,
}

impl<'a> GetAllResourceHandler<'a> {
    pub fn register(category_repo: &'a ResourceQueryRepository) -> Self {
        GetAllResourceHandler { category_repo: &category_repo }
    }
}

#[async_trait]
impl IQueryHandler<GetAllResourceQuery> for GetAllResourceHandler<'_>{
    fn get_name() -> String {
        String::from("Get All Resource")
    }

    type Output = Vec<ResourceResDto>;

    async fn query(&self, query: GetAllResourceQuery) -> Result<Self::Output, Error> {
        let result = self.category_repo
            .get_all()
            .await;
    
        match result {
            Ok(value) => Ok(value),
            _ => Err(ResourceGenericError::DBInternalError().into()),
        }
    }
}
