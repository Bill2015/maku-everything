use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;

use crate::command_from_dto;
use crate::modules::resource::application::dto::ResourceTaggingAttrPayloadDto;
use crate::modules::resource::domain::{ResourceGenericError, ResourceID};
use crate::modules::resource::repository::ResourceRepository;
use crate::modules::common::application::ICommandHandler;
use crate::modules::tag::repository::TagRepository;

mod dto;
pub use dto::*;

#[derive(Deserialize)]
pub struct ResourceUpdateTagCommand {
    pub id: String,

    pub tag_id: String,

    pub attrval: ResourceTaggingAttrPayloadDto,
}
command_from_dto!(ResourceUpdateTagCommand, ResourceUpdateTagDto);

// =====================================
pub struct ResourceUpdateTagHandler<'a> {
    resource_repo: &'a ResourceRepository<'a>,
    tag_repo: &'a TagRepository<'a>,
}

impl<'a> ResourceUpdateTagHandler<'a> {
    pub fn register(resource_repo: &'a ResourceRepository, tag_repo: &'a TagRepository) -> Self {
        Self { resource_repo, tag_repo }
    }
}

#[async_trait]
impl ICommandHandler<ResourceUpdateTagCommand> for ResourceUpdateTagHandler<'_> {

    fn get_name() -> String {
        String::from("Update Resource Tag Command")
    }

    type Output = ResourceID;

    async fn execute(&self, command: ResourceUpdateTagCommand) -> Result<Self::Output, Error> {
        let ResourceUpdateTagCommand { 
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
                
        // remove tag
        resource.get_mut_tagging().update_tag(&tag, attrval.into())?;
        
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
