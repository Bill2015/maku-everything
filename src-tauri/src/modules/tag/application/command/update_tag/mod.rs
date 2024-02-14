use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;

use crate::command_from_dto;
use crate::modules::tag::domain::{TagGenericError, TagID};
use crate::modules::tag::repository::TagRepository;
use crate::modules::common::application::ICommandHandler;

mod dto;
pub use dto::*;

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

    type Output = TagID;

    async fn execute(&self, command: UpdateTagCommand) -> Result<Self::Output, Error> {
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
            .or(Err(TagGenericError::DBInternalError()))?
            .ok_or(TagGenericError::IdNotFounded())?;

        // change name
        if name.is_some() {
            tag.change_name(name.unwrap())?;
        }

        // change description
        if description.is_some() {
            tag.change_description(description.unwrap());
        }

        // save
        let result = self.tag_repo
            .save(tag)
            .await;

        match result {
            Ok(value) => Ok(value.take_id()),
            _ => Err(TagGenericError::DBInternalError().into()),
        }
    }
}