use async_trait::async_trait;

use crate::common::application::IQueryHandler;
use crate::subject::domain::{SubjectError, SubjectGenericError};
use crate::subject::infrastructure::SubjectQueryBuilder;
use crate::subject::repository::SubjectQueryRepository;
use crate::subject::application::dto::SubjectResDto;

pub struct ListSubjectQuery { 
    pub id: Option<String>,

    pub name: Option<String>,

    pub belong_category: Option<String>, 

    pub order_by: Option<String>,
}

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

    type Output = Result<Vec<SubjectResDto>, SubjectError>;

    async fn query(&self, query: ListSubjectQuery) -> Self::Output {
        let query_builder = SubjectQueryBuilder::from(query);

        let result = self.subject_repo
            .query(query_builder)
            .await;
    
        match result {
            Ok(value) => Ok(value),
            _ => Err(SubjectError::Query(SubjectGenericError::DBInternalError())),
        }
    }
}
