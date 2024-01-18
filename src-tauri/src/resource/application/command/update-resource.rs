use async_trait::async_trait;
use serde::Deserialize;

use crate::command_from_dto;
use crate::resource::application::dto::UpdateResourceDto;
use crate::resource::domain::{ResourceError, ResourceGenericError};
use crate::resource::repository::ResourceRepository;
use crate::common::application::ICommandHandler;

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

    type Output = Result<String, ResourceError>;

    async fn execute(&self, command: UpdateResourceCommand) -> Self::Output {
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
            .or(Err(ResourceError::Update(ResourceGenericError::DBInternalError())))?
            .ok_or(ResourceError::Update(ResourceGenericError::IdNotFound()))?;

        // change name
        if name.is_some() {
            resource.change_name(name.unwrap());
        }

        // change description
        if description.is_some() {
            resource.change_description(description.unwrap());
        }

        // change auth
        if auth.is_some() {
            resource.set_authorize(auth.unwrap());
        }

        // save
        let reuslt = self.resource_repo
            .save(resource)
            .await;

        Ok(String::from("OK"))
    }
}