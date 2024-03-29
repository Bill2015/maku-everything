use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;

use crate::command_from_dto;
use crate::modules::common::application::IQueryHandler;
use crate::modules::common::infrastructure::QueryBuilder;
use crate::modules::resource::domain::ResourceGenericError;
use crate::modules::resource::infrastructure::ResourceQueryBuilder;
use crate::modules::resource::repository::ResourceQueryRepository;
use crate::modules::resource::application::dto::ResourceResDto;

mod dto;
pub use dto::ResourceListQueryDto;

#[derive(Deserialize)]
pub struct ListResourceQuery { 
    pub id: Option<String>,

    pub name: Option<String>,

    pub belong_category: Option<String>, 

    pub order_by: Option<String>,

    pub limit: Option<i64>,

    pub start: Option<i64>,
}
command_from_dto!(ListResourceQuery, ResourceListQueryDto);

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
        let builder_result = ResourceQueryBuilder::from(query).build()?;

        let result = self.resource_repo
            .query(builder_result)
            .await;
    
        match result {
            Ok(value) => Ok(value),
            _ => Err(ResourceGenericError::DBInternalError().into()),
        }
    }
}
