use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;

use crate::category::domain::CategoryID;
use crate::category::repository::CategoryRepository;
use crate::command_from_dto;
use crate::subject::application::dto::CreateSubjectDto;
use crate::subject::domain::{SubjectAggregate, SubjectGenericError, SubjectID};
use crate::subject::repository::SubjectRepository;
use crate::common::application::ICommandHandler;

#[derive(Deserialize)]
pub struct CreateSubjectCommand {
    pub name: String,

    pub description: String,

    pub belong_category: String,
}
command_from_dto!(CreateSubjectCommand, CreateSubjectDto);

// =====================================
pub struct CreateSubjectHandler<'a> {
    subject_repo: &'a SubjectRepository<'a>,
    category_repo: &'a CategoryRepository<'a>,
}

impl<'a> CreateSubjectHandler<'a> {
    pub fn register(subject_repo: &'a SubjectRepository, category_repo: &'a CategoryRepository<'a>) -> Self {
        Self { 
            subject_repo,
            category_repo,
        }
    }
}

#[async_trait]
impl ICommandHandler<CreateSubjectCommand> for CreateSubjectHandler<'_> {

    fn get_name() -> String {
        String::from("Create Subject Command")
    }

    type Output = SubjectID;

    async fn execute(&self, command: CreateSubjectCommand) -> Result<Self::Output, Error> {
        let CreateSubjectCommand { 
            name,
            description,
            belong_category,
        } = command;

        // get CategoryID
        let category_id = self.category_repo
            .is_exist(&belong_category)
            .await
            .then(|| CategoryID::from(belong_category))
            .ok_or(SubjectGenericError::BelongCategoryNotExists())?;

        // create new subject
        let new_subject = SubjectAggregate::new(name, description, category_id)?;

        // save
        let result = self.subject_repo
            .save(new_subject)
            .await;
        
        match result {
            Ok(value) => Ok(value.id),
            _ => Err(SubjectGenericError::DBInternalError().into()),
        }
    }
}
