use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;

use crate::command_from_dto;
use crate::modules::common::application::IQueryHandler;
use crate::modules::common::infrastructure::QueryBuilder;
use crate::modules::subject::domain::SubjectGenericError;
use crate::modules::subject::infrastructure::SubjectQueryBuilder;
use crate::modules::subject::repository::SubjectQueryRepository;
use crate::modules::subject::application::dto::{QuerySubjectDto, SubjectResDto};

#[derive(Deserialize)]
pub struct ListSubjectQuery { 
    pub id: Option<String>,

    pub name: Option<String>,

    pub belong_category: Option<String>, 

    pub order_by: Option<String>,
    
    pub limit: Option<i64>,

    pub start: Option<i64>,
}
command_from_dto!(ListSubjectQuery, QuerySubjectDto);

// =====================================
pub struct ListSubjectHandler<'a> {
    subject_repo: &'a SubjectQueryRepository<'a>,
}

impl<'a> ListSubjectHandler<'a> {
    pub fn register(subject_repo: &'a SubjectQueryRepository) -> Self {
        ListSubjectHandler { subject_repo: &subject_repo }
    }
}

#[async_trait]
impl IQueryHandler<ListSubjectQuery> for ListSubjectHandler<'_>{
    fn get_name() -> String {
        String::from("Get All Subject")
    }

    type Output = Vec<SubjectResDto>;

    async fn query(&self, query: ListSubjectQuery) -> Result<Self::Output, Error> {
        let builder_result = SubjectQueryBuilder::from(query).build()?;

        let result = self.subject_repo
            .query(builder_result)
            .await;
    
        match result {
            Ok(value) => Ok(value),
            _ => Err(SubjectGenericError::DBInternalError().into()),
        }
    }
}
