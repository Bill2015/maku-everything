use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;

use crate::command_from_dto;
use crate::resource::application::dto::ResourceAddTagDto;
use crate::resource::domain::{ResourceGenericError, ResourceID};
use crate::resource::repository::ResourceRepository;
use crate::common::application::ICommandHandler;
use crate::tag::domain::TagID;
use crate::tag::repository::TagRepository;

#[derive(Deserialize)]
pub struct ResourceAddTagCommand {
    pub id: String,
    pub tag_id: String,
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

        // add tag  
        resource.add_tag(tag_id)?;
        
        // save
        let result = self.resource_repo
            .save(resource)
            .await;
        
        match result {
            Ok(value) => Ok(value.id),
            _ => Err(ResourceGenericError::DBInternalError().into()),
        }
    }
}
