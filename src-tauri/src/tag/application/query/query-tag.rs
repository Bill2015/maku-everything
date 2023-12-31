use async_trait::async_trait;

use crate::common::application::IQueryHandler;
use crate::tag::domain::{TagError, TagGenericError};
use crate::tag::infrastructure::TagQueryBuilder;
use crate::tag::repository::TagQueryRepository;
use crate::tag::application::dto::TagResDto;

pub struct ListTagQuery { 
    pub id: Option<String>,

    pub name: Option<String>,

    pub belong_category: Option<String>, 

    pub belong_subject: Option<String>,

    pub tagging_resource: Option<String>,

    pub order_by: Option<String>,
}

// =====================================
pub struct ListTagHandler<'a> {
    tag_repo: &'a TagQueryRepository<'a>,
}

impl<'a> ListTagHandler<'a> {
    pub fn register(tag_repo: &'a TagQueryRepository) -> Self {
        ListTagHandler { tag_repo: &tag_repo }
    }
}

#[async_trait]
impl IQueryHandler<ListTagQuery> for ListTagHandler<'_>{
    fn get_name() -> String {
        String::from("Get All Tag")
    }

    type Output = Result<Vec<TagResDto>, TagError>;

    async fn query(&self, query: ListTagQuery) -> Self::Output {
        let query_builder = TagQueryBuilder::from(query);

        let result = self.tag_repo
            .query(query_builder)
            .await;
    
        match result {
            Ok(value) => Ok(value),
            _ => Err(TagError::Query(TagGenericError::DBInternalError())),
        }
    }
}
