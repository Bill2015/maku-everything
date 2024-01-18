
use crate::category::repository::{CategoryRepository, CATEGORY_REPOSITORY};
use crate::subject::domain::{SubjectError, SubjectGenericError};
use crate::subject::repository::{SUBJECT_REPOSITORY, SUBJECT_QUERY_REPOSITORY, SubjectRepository, SubjectQueryRepository};
use crate::subject::application::command::{CreateSubjectCommand, CreateSubjectHandler};
use crate::common::application::{ICommandHandler, IQueryHandler};

use super::command::{UpdateSubjectCommand, UpdateSubjectHandler};
use super::dto::{SubjectResDto, CreateSubjectDto, UpdateSubjectDto};
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

    pub async fn create_subject(&self, data: CreateSubjectDto) -> Result<String, SubjectError> {
        let command = CreateSubjectCommand::from(data);

        let result = CreateSubjectHandler::register(self.subject_repository, self.category_repository)
            .execute(command)
            .await?;

        Ok(result)
    }

    pub async fn update_subject(&self, data: UpdateSubjectDto) -> Result<String, SubjectError> {
        let command = UpdateSubjectCommand::from(data);

        let result = UpdateSubjectHandler::register(self.subject_repository)
            .execute(command)
            .await?;

        Ok(result)
    }

    pub async fn get_all_subject(&self) -> Result<Vec<SubjectResDto>, SubjectError> {
        let query = GetAllSubjectQuery { };

        let result = GetAllSubjectHandler::register(self.subject_queryrepo)
            .query(query)
            .await?;

        Ok(result)
    }

    pub async fn get_subject_by_id(&self, id: String) -> Result<Option<SubjectResDto>, SubjectError> {
        let query = GetByIdSubjectQuery { id: id };
        
        let result = GetByIdSubjectHandler::register(self.subject_queryrepo)
            .query(query)
            .await?;

        Ok(result)
    }

    pub async fn list_subjects(
        &self, 
        id: Option<String>,
        name: Option<String>,
        belong_category: Option<String>, 
        order_by: Option<String>,
    ) -> Result<Vec<SubjectResDto>, SubjectError> {
        let query = ListSubjectQuery { 
            id,
            name,
            belong_category,
            order_by
        };
        
        let result = ListSubjectHandler::register(self.subject_queryrepo)
            .query(query)
            .await?;

        Ok(result)
    }
}