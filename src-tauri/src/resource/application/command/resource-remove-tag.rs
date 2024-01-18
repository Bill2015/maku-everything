use async_trait::async_trait;
use serde::Deserialize;

use crate::command_from_dto;
use crate::resource::application::dto::ResourceRemoveTagDto;
use crate::resource::domain::{ResourceError, ResourceGenericError};
use crate::resource::repository::ResourceRepository;
use crate::common::application::ICommandHandler;
use crate::common::domain::ID;
use crate::tag::domain::TagID;
use crate::tag::repository::TagRepository;

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

    type Output = Result<String, ResourceError>;

    async fn execute(&self, command: ResourceRemoveTagCommand) -> Self::Output {
        let ResourceRemoveTagCommand { 
            id,
            tag_id,
        } = command;


        //get TagID
        let tag_id = self.tag_repo
            .is_exist(&tag_id)
            .await
            .then(|| TagID::from(tag_id))
            .ok_or(ResourceError::AddTag(ResourceGenericError::TagNotExists()))?;   

        // find by id
        let mut resource = self.resource_repo
            .find_by_id(id)
            .await
            .or(Err(ResourceError::AddTag(ResourceGenericError::DBInternalError())))?
            .ok_or(ResourceError::AddTag(ResourceGenericError::IdNotFound()))?;
                
        // remove tag
        resource.del_tag(tag_id)?;
        
        // save
        let result = self.resource_repo
            .save(resource)
            .await;
        
        match result {
            Ok(value) => Ok(value.id.to_string()),
            _ => Err(ResourceError::RemoveTag(ResourceGenericError::DBInternalError())),
        }
    }
}
