use std::fmt;
use async_trait::async_trait;

use crate::category::application::dto::CategoryError;
use crate::category::domain::CategoryAggregate;
use crate::category::repository::CategoryRepository;
use crate::common::application::ICommandHandler;
use crate::common::domain::ID;

pub struct CreateCategoryCommand {
    pub title: String,
    pub description: String,
    pub auth: bool,
}

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

    type Output = Result<String, CategoryError>;

    async fn execute(&self, command: CreateCategoryCommand) -> Self::Output {
        let CreateCategoryCommand { 
            title,
            description, 
            auth,
        } = command;

        // create new category
        let new_category = CategoryAggregate::new(title, description);

        // save
        let result = self.categroy_repo
            .save(new_category)
            .await;

        dbg!(&result);
        
        match result {
            Ok(value) => Ok(value.id.to_string()),
            _ => Err(CategoryError::Create()),
        }
    }
}
