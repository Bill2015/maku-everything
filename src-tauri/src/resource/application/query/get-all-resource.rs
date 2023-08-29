use std::fmt;
use async_trait::async_trait;

use crate::common::application::IQueryHandler;
use crate::resource::repository::ResourceQueryRepository;
use crate::resource::application::dto::ResourceResDto;

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

    type Output = Result<Vec<ResourceResDto>, String>;

    async fn query(&self, query: GetAllResourceQuery) -> Self::Output {
        let result = self.category_repo
            .get_all()
            .await;
    
        match result {
            Ok(value) => Ok(value),
            _ => Err(String::from("ResourceError::FindAll()")),
        }
    }
}
