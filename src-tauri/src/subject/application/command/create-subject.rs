use std::fmt;
use async_trait::async_trait;

use crate::category::domain::CategoryID;
use crate::subject::domain::{SubjectAggregate, SubjectError, SubjectGenericError};
use crate::subject::repository::SubjectRepository;
use crate::common::application::ICommandHandler;
use crate::common::domain::ID;

pub struct CreateSubjectCommand {
    pub name: String,

    pub description: String,

    pub belong_category: CategoryID,
}

// =====================================
pub struct CreateSubjectHandler<'a> {
    subject_repo: &'a SubjectRepository<'a>,
}

impl<'a> CreateSubjectHandler<'a> {
    pub fn register(subject_repo: &'a SubjectRepository) -> Self {
        CreateSubjectHandler { subject_repo: &subject_repo }
    }
}

#[async_trait]
impl ICommandHandler<CreateSubjectCommand> for CreateSubjectHandler<'_> {

    fn get_name() -> String {
        String::from("Create Subject Command")
    }

    type Output = Result<String, SubjectError>;

    async fn execute(&self, command: CreateSubjectCommand) -> Self::Output {
        let CreateSubjectCommand { 
            name,
            description,
            belong_category,
        } = command;

        // create new subject
        let new_subject = SubjectAggregate::new(name, description, belong_category)?;

        // save
        let result = self.subject_repo
            .save(new_subject)
            .await;
        
        match result {
            Ok(value) => Ok(value.id.to_string()),
            _ => Err(SubjectError::Create(SubjectGenericError::Unknown { message: String::from("Save Subject Failed") })),
        }
    }
}
