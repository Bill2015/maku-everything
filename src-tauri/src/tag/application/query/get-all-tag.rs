use anyhow::Error;
use async_trait::async_trait;

use crate::common::application::IQueryHandler;
use crate::tag::domain::TagGenericError;
use crate::tag::repository::TagQueryRepository;
use crate::tag::application::dto::TagResDto;

pub struct GetAllTagQuery { }

// =====================================
pub struct GetAllTagHandler<'a> {
    tag_repo: &'a TagQueryRepository<'a>,
}

impl<'a> GetAllTagHandler<'a> {
    pub fn register(tag_repo: &'a TagQueryRepository) -> Self {
        GetAllTagHandler { tag_repo: &tag_repo }
    }
}

#[async_trait]
impl IQueryHandler<GetAllTagQuery> for GetAllTagHandler<'_>{
    fn get_name() -> String {
        String::from("Get All Tag")
    }

    type Output = Vec<TagResDto>;

    async fn query(&self, query: GetAllTagQuery) -> Result<Self::Output, Error> {
        let result = self.tag_repo
            .get_all()
            .await;
    
        match result {
            Ok(value) => Ok(value),
            _ => Err(TagGenericError::DBInternalError().into()),
        }
    }
}
