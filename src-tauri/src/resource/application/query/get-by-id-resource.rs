use async_trait::async_trait;

use crate::common::application::IQueryHandler;
use crate::resource::repository::ResourceQueryRepository;
use crate::resource::application::dto::ResourceResDto;

pub struct GetByIdResourceQuery { 
    pub id: String,
}

// =====================================
pub struct GetByIdResourceHandler<'a> {
    resource_repo: &'a ResourceQueryRepository<'a>,
}

impl<'a> GetByIdResourceHandler<'a> {
    pub fn register(resource_repo: &'a ResourceQueryRepository) -> Self {
        GetByIdResourceHandler { resource_repo: &resource_repo }
    }
}

#[async_trait]
impl IQueryHandler<GetByIdResourceQuery> for GetByIdResourceHandler<'_>{
    fn get_name() -> String {
        String::from("Get All Resource")
    }

    type Output = Result<Option<ResourceResDto>, String>;

    async fn query(&self, query: GetByIdResourceQuery) -> Self::Output {
        let GetByIdResourceQuery { id } = query;

        let result = self.resource_repo
            .get_by_id(&id)
            .await;
    
        match result {
            Ok(value) => Ok(value),
            _ => Err(String::from("ResourceError::FindById(id)")),
        }
    }
}
