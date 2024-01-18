use async_trait::async_trait;
use serde::Deserialize;

use crate::command_from_dto;
use crate::tag::application::dto::UpdateTagDto;
use crate::tag::domain::{TagError, TagGenericError};
use crate::tag::repository::TagRepository;
use crate::common::application::ICommandHandler;

#[derive(Deserialize)]
pub struct UpdateTagCommand {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub auth: Option<bool>,
}

command_from_dto!(UpdateTagCommand, UpdateTagDto);

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
        let mut tag = self.tag_repo
            .find_by_id(&id)
            .await
            .or(Err(TagError::Update(TagGenericError::DBInternalError())))?
            .ok_or(TagError::Update(TagGenericError::IdNotFounded()))?;

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