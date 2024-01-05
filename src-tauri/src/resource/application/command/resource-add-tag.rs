use async_trait::async_trait;

use crate::resource::domain::{ResourceError, ResourceGenericError};
use crate::resource::repository::ResourceRepository;
use crate::common::application::ICommandHandler;
use crate::common::domain::ID;
use crate::tag::domain::TagID;

pub struct ResourceAddTagCommand {
    pub id: String,
    pub tag_id: TagID,
}

// =====================================
pub struct ResourceAddTagHandler<'a> {
    resource_repo: &'a ResourceRepository<'a>,
}

impl<'a> ResourceAddTagHandler<'a> {
    pub fn register(resource_repo: &'a ResourceRepository) -> Self {
        ResourceAddTagHandler { 
            resource_repo: &resource_repo,
        }
    }
}

#[async_trait]
impl ICommandHandler<ResourceAddTagCommand> for ResourceAddTagHandler<'_> {

    fn get_name() -> String {
        String::from("Create Resource Command")
    }

    type Output = Result<String, ResourceError>;

    async fn execute(&self, command: ResourceAddTagCommand) -> Self::Output {
        let ResourceAddTagCommand { 
            id,
            tag_id,
        } = command;


        // find by id
        let resource_result = self.resource_repo
            .find_by_id(id)
            .await;

        let mut resource = resource_result
            .ok()
            .flatten()
            .ok_or_else(|| ResourceError::AddTag(ResourceGenericError::IdNotFound()))?;

        // add tag  
        resource.add_tag(tag_id)?;
        
        // save
        let result = self.resource_repo
            .save(resource)
            .await;
        
        match result {
            Ok(value) => Ok(value.id.to_string()),
            _ => Err(ResourceError::AddTag(ResourceGenericError::Unknown { message: String::from("Save tag failed") })),
        }
    }
}
