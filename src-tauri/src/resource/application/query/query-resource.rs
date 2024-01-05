use std::fmt;
use async_trait::async_trait;

use crate::common::application::IQueryHandler;
use crate::resource::domain::{ResourceError, ResourceGenericError};
use crate::resource::infrastructure::ResourceQueryBuilder;
use crate::resource::repository::ResourceQueryRepository;
use crate::resource::application::dto::ResourceResDto;

pub struct ListResourceQuery { 
    pub id: Option<String>,

    pub name: Option<String>,

    pub belong_category: Option<String>, 

    pub order_by: Option<String>,
}

// =====================================
pub struct ListResourceHandler<'a> {
    resource_repo: &'a ResourceQueryRepository<'a>,
}

impl<'a> ListResourceHandler<'a> {
    pub fn register(resource_repo: &'a ResourceQueryRepository) -> Self {
        ListResourceHandler { resource_repo: &resource_repo }
    }
}

#[async_trait]
impl IQueryHandler<ListResourceQuery> for ListResourceHandler<'_>{
    fn get_name() -> String {
        String::from("Get All Resource")
    }

    type Output = Result<Vec<ResourceResDto>, ResourceError>;

    async fn query(&self, query: ListResourceQuery) -> Self::Output {
        let query_builder = ResourceQueryBuilder::from(query);

        let result = self.resource_repo
            .query(query_builder)
            .await;
    
        match result {
            Ok(value) => Ok(value),
            _ => Err(ResourceError::Query(ResourceGenericError::Unknown { message: String::from("Query parameter invalid") })),
        }
    }
}
