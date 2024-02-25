use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;

use crate::command_from_dto;
use crate::modules::resource::domain::{ResourceGenericError, ResourceID};
use crate::modules::resource::repository::ResourceRepository;
use crate::modules::common::application::ICommandHandler;

mod dto;
pub use dto::*;

#[derive(Deserialize)]
pub struct ResourceRenameFileCommand {
    pub id: String,

    pub new_name: Option<String>,
}
command_from_dto!(ResourceRenameFileCommand, ResourceRenameFileDto);

// =====================================
pub struct ResourceRenameFileHandler<'a> {
    resource_repo: &'a ResourceRepository<'a>,
}

impl<'a> ResourceRenameFileHandler<'a> {
    pub fn register(resource_repo: &'a ResourceRepository) -> Self {
        Self { resource_repo }
    }
}

#[async_trait]
impl ICommandHandler<ResourceRenameFileCommand> for ResourceRenameFileHandler<'_> {

    fn get_name() -> String {
        String::from("Create Resource Command")
    }

    type Output = ResourceID;

    async fn execute(&self, command: ResourceRenameFileCommand) -> Result<Self::Output, Error> {
        let ResourceRenameFileCommand { 
            id,
            new_name,
        } = command;

        // find by id
        let mut resource = self.resource_repo
            .find_by_id(id)
            .await
            .or(Err(ResourceGenericError::DBInternalError()))?
            .ok_or(ResourceGenericError::IdNotFound())?;

        // rename the file
        resource.rename_file(new_name)?;
        
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
