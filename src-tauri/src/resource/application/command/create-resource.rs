use std::fmt;
use async_trait::async_trait;

use crate::category::domain::CategoryID;
use crate::category::repository::CategoryRepository;
use crate::resource::domain::ResourceAggregate;
use crate::resource::repository::ResourceRepository;
use crate::common::application::ICommandHandler;
use crate::common::domain::ID;

pub struct CreateResourceCommand {
    pub title: String,
    pub description: String,
    pub belong_category: CategoryID,
    pub file_path: String,
}

// =====================================
pub struct CreateResourceHandler<'a> {
    resource_repo: &'a ResourceRepository<'a>,
}

impl<'a> CreateResourceHandler<'a> {
    pub fn register(resource_repo: &'a ResourceRepository) -> Self {
        CreateResourceHandler { 
            resource_repo: &resource_repo,
        }
    }
}

#[async_trait]
impl ICommandHandler<CreateResourceCommand> for CreateResourceHandler<'_> {

    fn get_name() -> String {
        String::from("Create Resource Command")
    }

    type Output = Result<String, String>;

    async fn execute(&self, command: CreateResourceCommand) -> Self::Output {
        let CreateResourceCommand { 
            title,
            description,
            belong_category, 
            file_path,
        } = command;


        // create new resource
        let new_resource = match ResourceAggregate::new(title, description, belong_category, file_path) {
            Ok(value) => value,
            _ => return Err(String::from("ResourceError::Create()")),
        };
        
        // save
        let result = self.resource_repo
            .save(new_resource)
            .await;
        
        match result {
            Ok(value) => Ok(value.id.to_string()),
            _ => Err(String::from("ResourceError::Create()")),
        }
    }
}
