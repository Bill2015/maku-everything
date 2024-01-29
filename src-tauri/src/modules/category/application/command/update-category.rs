use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;

use crate::modules::category::application::dto::UpdateCategoryDto;
use crate::modules::category::domain::{CategoryGenericError, CategoryID};
use crate::modules::category::repository::CategoryRepository;
use crate::modules::common::application::ICommandHandler;
use crate::command_from_dto;

#[derive(Deserialize)]
pub struct UpdateCategoryCommand {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub auth: Option<bool>,
}
command_from_dto!(UpdateCategoryCommand, UpdateCategoryDto);

// =====================================
pub struct UpdateCategoryHandler<'a> {
    categroy_repo: &'a CategoryRepository<'a>,
}

impl<'a> UpdateCategoryHandler<'a> {
    pub fn register(categroy_repo: &'a CategoryRepository) -> Self {
        UpdateCategoryHandler { categroy_repo: &categroy_repo }
    }
}

#[async_trait]
impl ICommandHandler<UpdateCategoryCommand> for UpdateCategoryHandler<'_> {

    fn get_name() -> String {
        String::from("Change Category Command")
    }

    type Output = CategoryID;

    async fn execute(&self, command: UpdateCategoryCommand) -> Result<Self::Output, Error> {
        let UpdateCategoryCommand { 
            id,
            name,
            description, 
            auth,
        } = command;

        // find by id
        let category_result = self.categroy_repo
            .find_by_id(&id)
            .await;

        let mut category = category_result
            .ok()
            .flatten()
            .ok_or_else(|| CategoryGenericError::IdNotFounded())?;
 
        // change name
        if name.is_some() {
            category.change_name(name.unwrap());
        }

        // change description
        if description.is_some() {
            category.change_description(description.unwrap());
        }

        // change auth
        if auth.is_some() {
            category.change_auth(auth.unwrap());
        }

        // save
        let result = self.categroy_repo
            .save(category)
            .await;

        match result {
            Ok(value) => Ok(value.id),
            _ => Err(CategoryGenericError::DBInternalError().into()),
        }
    }
}