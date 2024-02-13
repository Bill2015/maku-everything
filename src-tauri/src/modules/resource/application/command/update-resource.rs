use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;

use crate::command_from_dto;
use crate::modules::resource::application::dto::UpdateResourceDto;
use crate::modules::resource::domain::{ResourceGenericError, ResourceID};
use crate::modules::resource::repository::ResourceRepository;
use crate::modules::common::application::ICommandHandler;

#[derive(Deserialize)]
pub struct UpdateResourceCommand {
    pub id: String,

    pub name: Option<String>,

    pub description: Option<String>,

    pub auth: Option<bool>,
}
command_from_dto!(UpdateResourceCommand, UpdateResourceDto);

// =====================================
pub struct UpdateResourceHandler<'a> {
    resource_repo: &'a ResourceRepository<'a>,
}

impl<'a> UpdateResourceHandler<'a> {
    pub fn register(resource_repo: &'a ResourceRepository) -> Self {
        UpdateResourceHandler { resource_repo: &resource_repo }
    }
}

#[async_trait]
impl ICommandHandler<UpdateResourceCommand> for UpdateResourceHandler<'_> {

    fn get_name() -> String {
        String::from("Change Resource Command")
    }

    type Output = ResourceID;

    async fn execute(&self, command: UpdateResourceCommand) -> Result<Self::Output, Error> {
        let UpdateResourceCommand { 
            id,
            name,
            description, 
            auth,
        } = command;

        // find by id
        let  mut resource = self.resource_repo
            .find_by_id(id)
            .await
            .or(Err(ResourceGenericError::DBInternalError()))?
            .ok_or(ResourceGenericError::IdNotFound())?;

        // change name
        if name.is_some() {
            resource.change_name(name.unwrap())?;
        }

        // change description
        if description.is_some() {
            resource.change_description(description.unwrap())?;
        }

        // change auth
        if auth.is_some() {
            resource.set_authorize(auth.unwrap());
        }

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