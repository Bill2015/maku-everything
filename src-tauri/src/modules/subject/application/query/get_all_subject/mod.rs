use anyhow::Error;
use async_trait::async_trait;

use crate::modules::common::application::IQueryHandler;
use crate::modules::subject::domain::SubjectGenericError;
use crate::modules::subject::repository::SubjectQueryRepository;
use crate::modules::subject::application::dto::SubjectResDto;

pub struct GetAllSubjectQuery { }

// =====================================
pub struct GetAllSubjectHandler<'a> {
    subject_repo: &'a SubjectQueryRepository<'a>,
}

impl<'a> GetAllSubjectHandler<'a> {
    pub fn register(subject_repo: &'a SubjectQueryRepository) -> Self {
        GetAllSubjectHandler { subject_repo: &subject_repo }
    }
}

#[async_trait]
impl IQueryHandler<GetAllSubjectQuery> for GetAllSubjectHandler<'_>{
    fn get_name() -> String {
        String::from("Get All Subject")
    }

    type Output = Vec<SubjectResDto>;

    async fn query(&self, query: GetAllSubjectQuery) -> Result<Self::Output, Error> {
        let result = self.subject_repo
            .get_all()
            .await;
    
        match result {
            Ok(value) => Ok(value),
            _ => Err(SubjectGenericError::DBInternalError().into()),
        }
    }
}
