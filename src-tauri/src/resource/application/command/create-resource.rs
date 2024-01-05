use async_trait::async_trait;

use crate::category::domain::CategoryID;
use crate::resource::domain::{ResourceAggregate, ResourceError, ResourceGenericError};
use crate::resource::repository::ResourceRepository;
use crate::common::application::ICommandHandler;
use crate::common::domain::ID;

pub struct CreateResourceCommand {
    pub name: String,
    pub description: String,
    pub belong_category: CategoryID,
    pub root_path: String,
    pub file_path: String,
    pub url_path: String,
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

    type Output = Result<String, ResourceError>;

    async fn execute(&self, command: CreateResourceCommand) -> Self::Output {
        let CreateResourceCommand { 
            name,
            description,
            belong_category, 
            root_path,
            file_path,
            url_path,
        } = command;


        // create new resource
        let new_resource = ResourceAggregate::new(
            name,
            description,
            belong_category,
            root_path,
            file_path,
            url_path
        )?;
        
        // save
        let result = self.resource_repo
            .save(new_resource)
            .await;
        
        match result {
            Ok(value) => Ok(value.id.to_string()),
            _ => Err(ResourceError::Create(ResourceGenericError::DBInternalError())),
        }
    }
}
