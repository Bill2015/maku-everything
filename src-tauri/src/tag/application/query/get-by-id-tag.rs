use async_trait::async_trait;

use crate::common::application::IQueryHandler;
use crate::tag::domain::{TagError, TagGenericError};
use crate::tag::repository::TagQueryRepository;
use crate::tag::application::dto::TagResDto;

pub struct GetByIdTagQuery { 
    pub id: String,
}

// =====================================
pub struct GetByIdTagHandler<'a> {
    tag_repo: &'a TagQueryRepository<'a>,
}

impl<'a> GetByIdTagHandler<'a> {
    pub fn register(tag_repo: &'a TagQueryRepository) -> Self {
        GetByIdTagHandler { tag_repo: &tag_repo }
    }
}

#[async_trait]
impl IQueryHandler<GetByIdTagQuery> for GetByIdTagHandler<'_>{
    fn get_name() -> String {
        String::from("Get All Tag")
    }

    type Output = Result<Option<TagResDto>, TagError>;

    async fn query(&self, query: GetByIdTagQuery) -> Self::Output {
        let GetByIdTagQuery { id } = query;

        let result = self.tag_repo
            .get_by_id(&id)
            .await;
    
        match result {
            Ok(value) => Ok(value),
            _ => Err(TagError::GetById(TagGenericError::DBInternalError())),
        }
    }
}
