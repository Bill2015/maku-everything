use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;

use crate::modules::category::domain::CategoryFactory;
use crate::modules::category::domain::CategoryGenericError;
use crate::modules::category::domain::CategoryID;
use crate::modules::category::infrastructure::CategoryQueryBuilder;
use crate::modules::category::repository::CategoryRepository;
use crate::modules::common::application::ICommandHandler;
use crate::command_from_dto;
use crate::modules::common::infrastructure::QueryBuilder;

mod dto;
pub use dto::*;

#[derive(Deserialize)]
pub struct CreateCategoryCommand {
    pub name: String,

    pub description: String,

    pub root_path: String,
}
command_from_dto!(CreateCategoryCommand, CreateCategoryDto);

// =====================================
pub struct CreateCategoryHandler<'a> {
    categroy_repo: &'a CategoryRepository<'a>,
}

impl<'a> CreateCategoryHandler<'a> {
    pub fn register(categroy_repo: &'a CategoryRepository) -> Self {
        CreateCategoryHandler { categroy_repo: &categroy_repo }
    }
}

#[async_trait]
impl ICommandHandler<CreateCategoryCommand> for CreateCategoryHandler<'_> {

    fn get_name() -> String {
        String::from("Create Category Command")
    }

    type Output = CategoryID;

    async fn execute(&self, command: CreateCategoryCommand) -> Result<Self::Output, Error> {
        let CreateCategoryCommand { 
            name,
            description,
            root_path,
        } = command;

        // check name already existed
        let count = self.categroy_repo
            .get_by(CategoryQueryBuilder::new().set_name(name.clone()).build()?)
            .await
            .or(Err(CategoryGenericError::DBInternalError()))?
            .len();

        if count > 0 {
            return Err(CategoryGenericError::NameIsDuplicated { current_name: name }.into());
        }

        // create new category
        let new_category = CategoryFactory::create(name, description, root_path)?;

        // save
        let result = self.categroy_repo
            .save(new_category)
            .await;

        match result {
            Ok(value) => Ok(value.take_id()),
            _ => Err(CategoryGenericError::DBInternalError().into()),
        }
    }
}
