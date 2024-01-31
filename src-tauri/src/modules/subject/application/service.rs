
use anyhow::anyhow;

use crate::modules::category::repository::{CategoryRepository, CATEGORY_REPOSITORY};
use crate::modules::subject::domain::{SubjectError, SubjectID};
use crate::modules::subject::repository::{SUBJECT_REPOSITORY, SUBJECT_QUERY_REPOSITORY, SubjectRepository, SubjectQueryRepository};
use crate::modules::subject::application::command::{CreateSubjectCommand, CreateSubjectHandler};
use crate::modules::common::application::{ICommandHandler, IQueryHandler};

use super::command::*;
use super::dto::*;
use super::query::*;

pub static SUBJECT_SERVICE: SubjectService = SubjectService::init(
    &CATEGORY_REPOSITORY,
    &SUBJECT_REPOSITORY, 
    &SUBJECT_QUERY_REPOSITORY,
);

pub struct SubjectService<'a> {
    category_repository: &'a CategoryRepository<'a>,
    subject_repository: &'a SubjectRepository<'a>,
    subject_queryrepo: &'a SubjectQueryRepository<'a>,
}
impl<'a> SubjectService<'a> {
    const fn init(
        category_repository: &'a CategoryRepository<'a>,
        subject_repository: &'a SubjectRepository<'a>,
        subject_queryrepo: &'a SubjectQueryRepository<'a>
    ) -> Self {
        SubjectService { 
            category_repository: category_repository,
            subject_repository: subject_repository,
            subject_queryrepo: subject_queryrepo,
        }
    }

    pub async fn create_subject(&self, data: CreateSubjectDto) -> Result<SubjectID, SubjectError> {
        let command = CreateSubjectCommand::from(data);

        let result = CreateSubjectHandler::register(self.subject_repository, self.category_repository)
            .execute(command)
            .await
            .map_err(|err| SubjectError::Create(anyhow!(err)))?;

        Ok(result)
    }

    pub async fn update_subject(&self, data: UpdateSubjectDto) -> Result<SubjectID, SubjectError> {
        let command = UpdateSubjectCommand::from(data);

        let result = UpdateSubjectHandler::register(self.subject_repository)
            .execute(command)
            .await
            .map_err(|err| SubjectError::Update(anyhow!(err)))?;

        Ok(result)
    }

    pub async fn get_all_subject(&self) -> Result<Vec<SubjectResDto>, SubjectError> {
        let query = GetAllSubjectQuery { };

        let result = GetAllSubjectHandler::register(self.subject_queryrepo)
            .query(query)
            .await
            .map_err(|err| SubjectError::GetAll(anyhow!(err)))?;

        Ok(result)
    }

    pub async fn get_subject_by_id(&self, id: String) -> Result<Option<SubjectResDto>, SubjectError> {
        let query = GetByIdSubjectQuery { id: id };
        
        let result = GetByIdSubjectHandler::register(self.subject_queryrepo)
            .query(query)
            .await
            .map_err(|err| SubjectError::GetById(anyhow!(err)))?;

        Ok(result)
    }

    pub async fn list_subjects(&self, data: QuerySubjectDto) -> Result<Vec<SubjectResDto>, SubjectError> {
        let query = ListSubjectQuery::from(data);
        
        let result = ListSubjectHandler::register(self.subject_queryrepo)
            .query(query)
            .await
            .map_err(|err| SubjectError::Query(anyhow!(err)))?;

        Ok(result)
    }
}