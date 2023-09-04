use std::path::Path;
use std::process::Command;
use crate::common::application::{ICommandHandler, IQueryHandler};
use crate::category::repository::CategoryRepository;
use crate::resource::repository::{RESOURCE_REPOSITORY, ResourceRepository, ResourceQueryRepository, RESOURCE_QUERY_REPOSITORY};
use crate::category::repository::CATEGORY_REPOSITORY;
use crate::tag::repository::{TagRepository, TAG_REPOSITORY};

use super::command::*;
use super::dto::*;
use super::query::*;

pub static RESOURCE_SERVICE: ResourceService = ResourceService::init(
    &RESOURCE_REPOSITORY, 
    &RESOURCE_QUERY_REPOSITORY, 
    &CATEGORY_REPOSITORY,
    &TAG_REPOSITORY,
);

pub struct ResourceService<'a> {
    resource_repository: &'a ResourceRepository<'a>,
    resource_query_repo: &'a ResourceQueryRepository<'a>,
    category_repository: &'a CategoryRepository<'a>,
    tag_respository: &'a TagRepository<'a>,
}
impl<'a> ResourceService<'a> {
    pub const fn init(
        resource_repository: &'a ResourceRepository<'_>,
        resource_query_repo: &'a ResourceQueryRepository<'_>,
        category_repository: &'a CategoryRepository<'_>,
        tag_respository: &'a TagRepository<'_>,
    ) -> Self {
        ResourceService { 
            resource_repository: resource_repository,
            resource_query_repo: resource_query_repo,
            category_repository: category_repository,
            tag_respository: tag_respository,
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

        let _ = handler.execute(command).await?;

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
        
        let _  = handler.execute(command).await?;

        Ok(String::from("Ok"))
    }

    pub async fn add_resource_tag(&self, resource_id: String, tag_id: String) -> Result<String, String> {
        // Category
        let tag = self.tag_respository
            .find_by_id(&tag_id)
            .await
            .unwrap_or(None);

        if tag.is_none() {
            println!("Tag Not Exist");
            return Err(String::from("Tag Not Exist"));
        }
        
        let command = ResourceAddTagCommand {
            id: resource_id,
            tag_id: tag.unwrap().id,
        };

        let handler = ResourceAddTagHandler::register(self.resource_repository);
        
        let _  = handler.execute(command).await?;

        Ok(String::from("Ok"))
    }

    pub async fn remove_resource_tag(&self, resource_id: String, tag_id: String) -> Result<String, String> {
        // Category
        let tag = self.tag_respository
            .find_by_id(&tag_id)
            .await
            .unwrap_or(None);

        if tag.is_none() {
            println!("Tag Not Exist");
            return Err(String::from("Tag Not Exist"));
        }
        
        let command = ResourceRemoveTagCommand {
            id: resource_id,
            tag_id: tag.unwrap().id,
        };

        let handler = ResourceRemoveTagHandler::register(self.resource_repository);
        
        let _  = handler.execute(command).await?;

        Ok(String::from("Ok"))
    }



    pub async fn get_resource_by_id(&self, resource_id: String) -> Result<Option<ResourceResDto>, String> {
        let query = GetByIdResourceQuery { id: resource_id };

        let handler = GetByIdResourceHandler::register(self.resource_query_repo);

        let res = handler.query(query).await?;

        Ok(res)
    }

    pub async fn get_all_resource(&self) -> Result<Vec<ResourceResDto>, String> {
        let query = GetAllResourceQuery {};

        let handler = GetAllResourceHandler::register(self.resource_query_repo);

        let res = handler.query(query).await?;

        Ok(res)
    }

    pub async fn resource_detail(&self, resource_id: String) -> Result<Option<ResourceDetailDto>, String> {
        let query = ResourceDetailQuery { id: resource_id };

        let handler = ResourceDetailHandler::register(self.resource_query_repo);

        let res = handler.query(query).await?;

        Ok(res)
    }

    pub async fn expore_the_file(&self, file_path: String) -> Result<(), String> {
        let path = Path::new(file_path.as_str());

        if path.exists() == false {
            return Err(String::from("File Not Existed"));
        }
        // TODO: For now, Windows Only 
        Command::new("explorer")
            .args(["/select,", file_path.as_str()]) // The comma after select is not a typo
            .spawn()
            .unwrap();

        Ok(())
    }

    pub async fn list_resource(
        &self, 
        id: Option<String>,
        name: Option<String>,
        belong_category: Option<String>, 
        order_by: Option<String>,
    ) -> Result<Vec<ResourceResDto>, String> {
        let query = ListResourceQuery { 
            id,
            name,
            belong_category,
            order_by
        };
        
        let handler = ListResourceHandler::register(self.resource_query_repo);

        let res = handler.query(query).await?;

        Ok(res)
    }
}