use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;

use crate::command_from_dto;
use crate::modules::resource::application::dto::ResourceRemoveTagDto;
use crate::modules::resource::domain::{ResourceGenericError, ResourceID};
use crate::modules::resource::repository::ResourceRepository;
use crate::modules::common::application::ICommandHandler;
use crate::modules::tag::domain::TagID;
use crate::modules::tag::repository::TagRepository;

#[derive(Deserialize)]
pub struct ResourceRemoveTagCommand {
    pub id: String,

    pub tag_id: String,
}
command_from_dto!(ResourceRemoveTagCommand, ResourceRemoveTagDto);

// =====================================
pub struct ResourceRemoveTagHandler<'a> {
    resource_repo: &'a ResourceRepository<'a>,
    tag_repo: &'a TagRepository<'a>,
}

impl<'a> ResourceRemoveTagHandler<'a> {
    pub fn register(resource_repo: &'a ResourceRepository, tag_repo: &'a TagRepository) -> Self {
        Self { resource_repo, tag_repo }
    }
}

#[async_trait]
impl ICommandHandler<ResourceRemoveTagCommand> for ResourceRemoveTagHandler<'_> {

    fn get_name() -> String {
        String::from("Create Resource Command")
    }

    type Output = ResourceID;

    async fn execute(&self, command: ResourceRemoveTagCommand) -> Result<Self::Output, Error> {
        let ResourceRemoveTagCommand { 
            id,
            tag_id,
        } = command;


        //get TagID
        let tag_id = self.tag_repo
            .is_exist(&tag_id)
            .await
            .then(|| TagID::from(tag_id))
            .ok_or(ResourceGenericError::TagNotExists())?;   

        // find by id
        let mut resource = self.resource_repo
            .find_by_id(id)
            .await
            .or(Err(ResourceGenericError::DBInternalError()))?
            .ok_or(ResourceGenericError::IdNotFound())?;
                
        // remove tag
        resource.get_mut_tagging().del_tag(&tag_id)?;
        
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
