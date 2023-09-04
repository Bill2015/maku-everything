use async_trait::async_trait;

use crate::subject;
use crate::subject::domain::SubjectAggregate;
use crate::subject::repository::SubjectRepository;
use crate::common::application::ICommandHandler;

pub struct UpdateSubjectCommand {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub auth: Option<bool>,
}

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
        String::from("Change Subject Command")
    }

    type Output = Result<String, String>;

    async fn execute(&self, command: UpdateSubjectCommand) -> Self::Output {
        let UpdateSubjectCommand { 
            id,
            name,
            description, 
            auth,
        } = command;

        // find by id
        let subject_result = self.subject_repo
            .find_by_id(&id)
            .await;

        let mut subject = subject_result
            .ok()
            .flatten()
            .ok_or_else(|| String::from("SubjectError::Update(id)"))?;
 
        // change name
        if name.is_some() {
            subject.change_name(name.unwrap());
        }

        // change description
        if description.is_some() {
            subject.change_description(description.unwrap());
        }

        // save
        let reuslt = self.subject_repo
            .save(subject)
            .await;

        Ok(String::from("OK"))
    }
}