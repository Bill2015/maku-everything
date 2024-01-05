use async_trait::async_trait;

use crate::tag::domain::{TagError, TagGenericError};
use crate::tag::repository::TagRepository;
use crate::common::application::ICommandHandler;

pub struct UpdateTagCommand {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub auth: Option<bool>,
}

// =====================================
pub struct UpdateTagHandler<'a> {
    tag_repo: &'a TagRepository<'a>,
}

impl<'a> UpdateTagHandler<'a> {
    pub fn register(tag_repo: &'a TagRepository) -> Self {
        UpdateTagHandler { tag_repo: &tag_repo }
    }
}

#[async_trait]
impl ICommandHandler<UpdateTagCommand> for UpdateTagHandler<'_> {

    fn get_name() -> String {
        String::from("Change Tag Command")
    }

    type Output = Result<String, TagError>;

    async fn execute(&self, command: UpdateTagCommand) -> Self::Output {
        let UpdateTagCommand { 
            id,
            name,
            description, 
            auth,
        } = command;

        // find by id
        let tag_result = self.tag_repo
            .find_by_id(&id)
            .await;

        let mut tag = tag_result
            .ok()
            .flatten()
            .ok_or_else(|| TagError::Update(TagGenericError::IdNotFounded()))?;
 
        // change name
        if name.is_some() {
            tag.change_name(name.unwrap())?;
        }

        // change description
        if description.is_some() {
            tag.change_description(description.unwrap());
        }

        // save
        let reuslt = self.tag_repo
            .save(tag)
            .await;

        Ok(String::from("OK"))
    }
}