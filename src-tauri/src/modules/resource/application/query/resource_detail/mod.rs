use anyhow::Error;
use async_trait::async_trait;

use crate::modules::common::application::IQueryHandler;
use crate::modules::resource::domain::ResourceGenericError;
use crate::modules::resource::repository::ResourceQueryRepository;

mod dto;
pub use dto::*;


pub struct ResourceDetailQuery { 
    pub id: String,
}

// =====================================
pub struct ResourceDetailHandler<'a> {
    resource_repo: &'a ResourceQueryRepository<'a>,
}

impl<'a> ResourceDetailHandler<'a> {
    pub fn register(resource_repo: &'a ResourceQueryRepository) -> Self {
        ResourceDetailHandler { resource_repo: &resource_repo }
    }
}

#[async_trait]
impl IQueryHandler<ResourceDetailQuery> for ResourceDetailHandler<'_>{
    fn get_name() -> String {
        String::from("Get All Resource")
    }

    type Output = Option<ResourceDetailDto>;

    async fn query(&self, query: ResourceDetailQuery) -> Result<Self::Output, Error> {
        let ResourceDetailQuery { id } = query;

        let result = self.resource_repo
            .detail(&id)
            .await;

        match result {
            Ok(value) => Ok(value),
            _ => Err(ResourceGenericError::DBInternalError().into()),
        }
    }
}
