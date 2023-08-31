use std::fmt;
use async_trait::async_trait;

use crate::resource::domain::{ResourceAggregate, ResourceID};
use crate::resource::repository::ResourceRepository;
use crate::common::application::ICommandHandler;
use crate::common::domain::ID;
use crate::tag::domain::TagID;

pub struct ResourceRemoveTagCommand {
    pub id: String,
    pub tag_id: TagID,
}

// =====================================
pub struct ResourceRemoveTagHandler<'a> {
    resource_repo: &'a ResourceRepository<'a>,
}

impl<'a> ResourceRemoveTagHandler<'a> {
    pub fn register(resource_repo: &'a ResourceRepository) -> Self {
        ResourceRemoveTagHandler { 
            resource_repo: &resource_repo,
        }
    }
}

#[async_trait]
impl ICommandHandler<ResourceRemoveTagCommand> for ResourceRemoveTagHandler<'_> {

    fn get_name() -> String {
        String::from("Create Resource Command")
    }

    type Output = Result<String, String>;

    async fn execute(&self, command: ResourceRemoveTagCommand) -> Self::Output {
        let ResourceRemoveTagCommand { 
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
            .ok_or_else(|| String::from("ResourceError::Update(id)"))?;
                
        // remove tag
        resource.del_tag(tag_id)?;
        
        // save
        let result = self.resource_repo
            .save(resource)
            .await;
        
        match result {
            Ok(value) => Ok(value.id.to_string()),
            _ => Err(String::from("ResourceError::Create()")),
        }
    }
}
