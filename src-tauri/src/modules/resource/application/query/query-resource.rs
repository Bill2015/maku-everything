use anyhow::Error;
use async_trait::async_trait;

use crate::modules::common::application::IQueryHandler;
use crate::modules::resource::domain::ResourceGenericError;
use crate::modules::resource::infrastructure::ResourceQueryBuilder;
use crate::modules::resource::repository::ResourceQueryRepository;
use crate::modules::resource::application::dto::ResourceResDto;

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

    type Output = Vec<ResourceResDto>;

    async fn query(&self, query: ListResourceQuery) -> Result<Self::Output, Error> {
        let query_builder = ResourceQueryBuilder::from(query);

        let result = self.resource_repo
            .query(query_builder)
            .await;
    
        match result {
            Ok(value) => Ok(value),
            _ => Err(ResourceGenericError::DBInternalError().into()),
        }
    }
}
