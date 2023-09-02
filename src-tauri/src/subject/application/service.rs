
use crate::category::repository::{CategoryRepository, CATEGORY_REPOSITORY};
use crate::subject::repository::{SUBJECT_REPOSITORY, SUBJECT_QUERY_REPOSITORY, SubjectRepository, SubjectQueryRepository};
use crate::subject::application::command::{CreateSubjectCommand, CreateSubjectHandler};
use crate::common::application::{ICommandHandler, IQueryHandler};

use super::command::{UpdateSubjectCommand, UpdateSubjectHandler};
use super::dto::SubjectResDto;
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

    pub async fn create_subject(&self, name: String, description: String, belong_category: String) -> Result<String, String> {
        let category = self.category_repository
            .find_by_id(&belong_category)
            .await
            .unwrap_or(None);

        if category.is_none() {
            println!("Category Not Exist");
            return Err(String::from("Category Not Exist"));
        }

        let command = CreateSubjectCommand {
            name: name,
            description: description,
            belong_category: category.unwrap().id,
        };
        let handler = CreateSubjectHandler::register(self.subject_repository);
        
        let res = handler.execute(command).await?;

        Ok(res)
    }

    pub async fn update_subject(&self, id: String, name: Option<String>, description: Option<String>, auth: Option<bool>) -> Result<String, String> {
        let command = UpdateSubjectCommand {
            id: id,
            name: name,
            description: description,
            auth: auth,
        };

        let handler = UpdateSubjectHandler::register(self.subject_repository);

        let res = handler.execute(command).await?;

        Ok(res)
    }

    pub async fn get_all_subject(&self) -> Result<Vec<SubjectResDto>, String> {
        let query = GetAllSubjectQuery { };

        let handler = GetAllSubjectHandler::register(self.subject_queryrepo);

        let res = handler.query(query).await?;

        Ok(res)
    }

    pub async fn get_subject_by_id(&self, id: String) -> Result<Option<SubjectResDto>, String> {
        let query = GetByIdSubjectQuery { id: id };
        
        let handler = GetByIdSubjectHandler::register(self.subject_queryrepo);

        let res = handler.query(query).await?;

        Ok(res)
    }

    pub async fn list_subjects(
        &self, 
        id: Option<String>,
        name: Option<String>,
        belong_category: Option<String>, 
        order_by: Option<String>,
    ) -> Result<Vec<SubjectResDto>, String> {
        let query = ListSubjectQuery { 
            id,
            name,
            belong_category,
            order_by
        };
        
        let handler = ListSubjectHandler::register(self.subject_queryrepo);

        let res = handler.query(query).await?;

        Ok(res)
    }
}