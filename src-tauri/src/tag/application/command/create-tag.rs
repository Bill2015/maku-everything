use async_trait::async_trait;

use crate::category::domain::CategoryID;
use crate::subject::domain::SubjectID;
use crate::tag::domain::{TagAggregate, TagError, TagGenericError};
use crate::tag::repository::TagRepository;
use crate::common::application::ICommandHandler;
use crate::common::domain::ID;

pub struct CreateTagCommand {
    pub name: String,

    pub description: String,

    pub belong_category: CategoryID,

    pub belong_subject: SubjectID,
}

// =====================================
pub struct CreateTagHandler<'a> {
    tag_repo: &'a TagRepository<'a>,
}

impl<'a> CreateTagHandler<'a> {
    pub fn register(tag_repo: &'a TagRepository) -> Self {
        CreateTagHandler { tag_repo: &tag_repo }
    }
}

#[async_trait]
impl ICommandHandler<CreateTagCommand> for CreateTagHandler<'_> {

    fn get_name() -> String {
        String::from("Create Tag Command")
    }

    type Output = Result<String, TagError>;

    async fn execute(&self, command: CreateTagCommand) -> Self::Output {
        let CreateTagCommand { 
            name,
            description,
            belong_category,
            belong_subject,
        } = command;

        // create new tag
        let new_tag = TagAggregate::new(name, description, belong_category, belong_subject)?;

        // save
        let result = self.tag_repo
            .save(new_tag)
            .await;
        
        match result {
            Ok(value) => Ok(value.id.to_string()),
            _ => Err(TagError::Create(TagGenericError::Unknown { message: String::from("Database Error") })),
        }
    }
}
