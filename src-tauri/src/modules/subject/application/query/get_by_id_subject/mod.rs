use anyhow::Error;
use async_trait::async_trait;

use crate::modules::common::application::IQueryHandler;
use crate::modules::subject::domain::SubjectGenericError;
use crate::modules::subject::repository::SubjectQueryRepository;
use crate::modules::subject::application::dto::SubjectResDto;

pub struct GetByIdSubjectQuery { 
    pub id: String,
}

// =====================================
pub struct GetByIdSubjectHandler<'a> {
    subject_repo: &'a SubjectQueryRepository<'a>,
}

impl<'a> GetByIdSubjectHandler<'a> {
    pub fn register(subject_repo: &'a SubjectQueryRepository) -> Self {
        GetByIdSubjectHandler { subject_repo: &subject_repo }
    }
}

#[async_trait]
impl IQueryHandler<GetByIdSubjectQuery> for GetByIdSubjectHandler<'_>{
    fn get_name() -> String {
        String::from("Get All Subject")
    }

    type Output = Option<SubjectResDto>;

    async fn query(&self, query: GetByIdSubjectQuery) -> Result<Self::Output, Error> {
        let GetByIdSubjectQuery { id } = query;

        let result = self.subject_repo
            .get_by_id(&id)
            .await;
    
        match result {
            Ok(value) => Ok(value),
            _ => Err(SubjectGenericError::DBInternalError().into()),
        }
    }
}
