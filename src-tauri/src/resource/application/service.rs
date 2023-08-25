
use crate::category::application::command::UpdateCategoryCommand;
use crate::category::repository::CategoryRepository;
use crate::common::application::ICommandHandler;
use crate::resource::domain::ResourceAggregate;
use crate::resource::repository::{RESOURCE_REPOSITORY, ResourceRepository};
use crate::category::repository::CATEGORY_REPOSITORY;

use super::command::{CreateResourceCommand, CreateResourceHandler, UpdateResourceHandler, UpdateResourceCommand};

pub static RESOURCE_SERVICE: ResourceService = ResourceService::init(&RESOURCE_REPOSITORY, &CATEGORY_REPOSITORY);

pub struct ResourceService<'a> {
    resource_repository: &'a ResourceRepository<'a>,
    category_repository: &'a CategoryRepository<'a>,
}
impl<'a> ResourceService<'a> {
    pub const fn init(
        resource_repository: &'a ResourceRepository<'_>,
        category_repository: &'a CategoryRepository<'_>,
    ) -> Self {
        ResourceService { 
            resource_repository: resource_repository,
            category_repository: category_repository,
        }
    }

    pub async fn create_resource(&self, title: String, description: String, file_path: String, belong_category: String) -> Result<String, String> {
        let category = self.category_repository
            .find_by_id(&belong_category)
            .await
            .unwrap_or(None);

        if category.is_none() {
            println!("Category Not Exist");
            return Err(String::from("Category Not Exist"));
        }
        
        let command = CreateResourceCommand {
            title,
            description,
            belong_category: category.unwrap().id,
            file_path,
        };

        let handler = CreateResourceHandler::register(self.resource_repository);

        let _ = handler.execute(command);

        Ok(String::from("Ok"))
    }

    pub async fn update_resource(&self, id: String, title: Option<String>, description: Option<String>, auth: Option<bool>) -> Result<String, String> {
        let command = UpdateResourceCommand {
            id,
            title,
            description,
            auth,
        };

        let handler = UpdateResourceHandler::register(self.resource_repository);
        
        let _  = handler.execute(command);

        Ok(String::from("Ok"))
    }

    pub async fn add_tag(resource_id: String, tag_id: String) {
        // do add tags
    }
}