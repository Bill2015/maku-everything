use anyhow::Error;
use async_trait::async_trait;

use crate::modules::common::application::IQueryHandler;
use crate::modules::tag::domain::TagGenericError;
use crate::modules::tag::infrastructure::TagQueryBuilder;
use crate::modules::tag::repository::TagQueryRepository;
use crate::modules::tag::application::dto::TagResDto;

pub struct ListTagQuery { 
    pub id: Option<String>,

    pub name: Option<String>,

    pub belong_category: Option<String>, 

    pub belong_subject: Option<String>,

    pub belong_subject_name: Option<String>,

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

    type Output = Vec<TagResDto>;

    async fn query(&self, query: ListTagQuery) -> Result<Self::Output, Error> {
        let query_builder = TagQueryBuilder::from(query);

        let result = self.tag_repo
            .query(query_builder)
            .await;
    
        match result {
            Ok(value) => Ok(value),
            _ => Err(TagGenericError::DBInternalError().into()),
        }
    }
}
