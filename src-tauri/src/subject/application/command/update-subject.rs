use async_trait::async_trait;
use serde::Deserialize;

use crate::command_from_dto;
use crate::subject::application::dto::UpdateSubjectDto;
use crate::subject::domain::{SubjectError, SubjectGenericError, SubjectID};
use crate::subject::repository::SubjectRepository;
use crate::common::application::ICommandHandler;

#[derive(Deserialize)]
pub struct UpdateSubjectCommand {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub auth: Option<bool>,
}
command_from_dto!(UpdateSubjectCommand, UpdateSubjectDto);

// =====================================
pub struct UpdateSubjectHandler<'a> {
    subject_repo: &'a SubjectRepository<'a>,
}

impl<'a> UpdateSubjectHandler<'a> {
    pub fn register(subject_repo: &'a SubjectRepository) -> Self {
        UpdateSubjectHandler { subject_repo: &subject_repo }
    }
}

#[async_trait]
impl ICommandHandler<UpdateSubjectCommand> for UpdateSubjectHandler<'_> {

    fn get_name() -> String {
        String::from("Update Subject Command")
    }

    type Output = Result<SubjectID, SubjectError>;

    async fn execute(&self, command: UpdateSubjectCommand) -> Self::Output {
        let UpdateSubjectCommand { 
            id,
            name,
            description, 
            auth,
        } = command;

        // find by id
        let mut subject = self.subject_repo
            .find_by_id(&id)
            .await
            .or(Err(SubjectError::Update(SubjectGenericError::DBInternalError())))?
            .ok_or(SubjectError::Update(SubjectGenericError::IdNotFounded()))?;

        // change name
        if name.is_some() {
            subject.change_name(name.unwrap())?;
        }

        // change description
        if description.is_some() {
            subject.change_description(description.unwrap());
        }

        // save
        let result = self.subject_repo
            .save(subject)
            .await;

        match result {
            Ok(value) => Ok(value.id),
            _ => Err(SubjectError::Update(SubjectGenericError::DBInternalError())),
        }    
    }
}