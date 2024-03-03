use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;

use crate::command_from_dto;
use crate::modules::category::domain::CategoryID;
use crate::modules::category::repository::CategoryRepository;
use crate::modules::common::infrastructure::QueryBuilder;
use crate::modules::subject::domain::{SubjectFactory, SubjectGenericError, SubjectID};
use crate::modules::subject::infrastructure::SubjectQueryBuilder;
use crate::modules::subject::repository::SubjectRepository;
use crate::modules::common::application::ICommandHandler;

mod dto;
pub use dto::*;

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
            .then(|| CategoryID::from(belong_category.clone()))
            .ok_or(SubjectGenericError::BelongCategoryNotExists())?;

        // check name existed
        let count = self.subject_repo
            .get_by(
                SubjectQueryBuilder::new()
                    .set_name(name.clone())
                    .set_belong_category(belong_category)
                    .build()?
            )
            .await
            .or(Err(SubjectGenericError::DBInternalError()))?
            .len();

        if count > 0 {
            return Err(SubjectGenericError::NameIsDuplicated { current_name: name }.into());
        }

        // create new subject
        let new_subject = SubjectFactory::create(name, description, &category_id)?;

        // save
        let result = self.subject_repo
            .save(new_subject)
            .await;
        
        match result {
            Ok(value) => Ok(value.take_id()),
            _ => Err(SubjectGenericError::DBInternalError().into()),
        }
    }
}
