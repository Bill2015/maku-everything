use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;

use crate::command_from_dto;
use crate::modules::common::application::IQueryHandler;
use crate::modules::common::infrastructure::QueryBuilder;
use crate::modules::tag::domain::TagGenericError;
use crate::modules::tag::infrastructure::TagQueryBuilder;
use crate::modules::tag::repository::TagQueryRepository;
use crate::modules::tag::application::dto::TagResDto;

mod dto;
pub use dto::*;

#[derive(Deserialize)]
pub struct ListTagQuery { 
    pub id: Option<String>,

    pub name: Option<String>,

    pub belong_category: Option<String>, 

    pub belong_subject: Option<String>,

    pub belong_subject_name: Option<String>,

    pub tagging_resource: Option<String>,

    pub order_by: Option<String>,

    pub limit: Option<i64>,

    pub start: Option<i64>,
}
command_from_dto!(ListTagQuery, QueryTagDto);

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
        let builder_result = TagQueryBuilder::from(query).build()?;

        let result = self.tag_repo
            .query(builder_result)
            .await;
    
        match result {
            Ok(value) => Ok(value),
            _ => Err(TagGenericError::DBInternalError().into()),
        }
    }
}
