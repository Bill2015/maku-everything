use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;

use crate::command_from_dto;
use crate::modules::resource::application::dto::ResourceTaggingAttrPayloadDto;
use crate::modules::resource::domain::entities::TaggingAttrPayload;
use crate::modules::resource::domain::{ResourceGenericError, ResourceID};
use crate::modules::resource::repository::ResourceRepository;
use crate::modules::common::application::ICommandHandler;
use crate::modules::tag::repository::TagRepository;

mod dto;
pub use dto::*;

#[derive(Deserialize)]
pub struct ResourceAddTagCommand {
    pub id: String,
    pub tag_id: String,
    pub attrval: Option<ResourceTaggingAttrPayloadDto>,
}
command_from_dto!(ResourceAddTagCommand, ResourceAddTagDto);

// =====================================
pub struct ResourceAddTagHandler<'a> {
    resource_repo: &'a ResourceRepository<'a>,
    tag_repo: &'a TagRepository<'a>,
}

impl<'a> ResourceAddTagHandler<'a> {
    pub fn register(resource_repo: &'a ResourceRepository, tag_repo: &'a TagRepository) -> Self {
        Self { resource_repo, tag_repo }
    }
}

#[async_trait]
impl ICommandHandler<ResourceAddTagCommand> for ResourceAddTagHandler<'_> {

    fn get_name() -> String {
        String::from("Create Resource Command")
    }

    type Output = ResourceID;

    async fn execute(&self, command: ResourceAddTagCommand) -> Result<Self::Output, Error> {
        let ResourceAddTagCommand { 
            id,
            tag_id,
            attrval,
        } = command;

        //get TagID
        let tag = self.tag_repo
            .find_by_id(&tag_id)
            .await
            .or(Err(ResourceGenericError::DBInternalError()))?
            .ok_or(ResourceGenericError::TagNotExists())?;

        // find by id
        let mut resource = self.resource_repo
            .find_by_id(id)
            .await
            .or(Err(ResourceGenericError::DBInternalError()))?
            .ok_or(ResourceGenericError::IdNotFound())?;

        // add tag
        let payload: Option<TaggingAttrPayload> = attrval.map(|x| x.into());
        resource.get_mut_tagging().add_tag(&tag, payload)?;
        
        // save
        let result = self.resource_repo
            .save(resource)
            .await;
        
        match result {
            Ok(value) => Ok(value.take_id()),
            _ => Err(ResourceGenericError::DBInternalError().into()),
        }
    }
}
