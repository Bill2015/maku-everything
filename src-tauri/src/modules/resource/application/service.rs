use std::path::Path;
use std::process::Command;

use anyhow::anyhow;

use crate::modules::common::application::{ICommandHandler, IQueryHandler};
use crate::modules::category::repository::CategoryRepository;
use crate::modules::resource::domain::{ResourceError, ResourceGenericError, ResourceID};
use crate::modules::resource::repository::{RESOURCE_REPOSITORY, ResourceRepository, ResourceQueryRepository, RESOURCE_QUERY_REPOSITORY};
use crate::modules::category::repository::CATEGORY_REPOSITORY;
use crate::modules::tag::repository::{TagRepository, TAG_REPOSITORY, TagQueryRepository, TAG_QUERY_REPOSITORY};

use super::command::*;
use super::dto::*;
use super::query::*;

pub static RESOURCE_SERVICE: ResourceService = ResourceService::init(
    &RESOURCE_REPOSITORY, 
    &RESOURCE_QUERY_REPOSITORY, 
    &CATEGORY_REPOSITORY,
    &TAG_REPOSITORY,
    &TAG_QUERY_REPOSITORY,
);

pub struct ResourceService<'a> {
    resource_repository: &'a ResourceRepository<'a>,
    resource_query_repo: &'a ResourceQueryRepository<'a>,
    category_repository: &'a CategoryRepository<'a>,
    tag_respository: &'a TagRepository<'a>,
    tag_query_repository: &'a TagQueryRepository<'a>,
}
impl<'a> ResourceService<'a> {
    pub const fn init(
        resource_repository: &'a ResourceRepository<'_>,
        resource_query_repo: &'a ResourceQueryRepository<'_>,
        category_repository: &'a CategoryRepository<'_>,
        tag_respository: &'a TagRepository<'_>,
        tag_query_repository: &'a TagQueryRepository<'a>,
    ) -> Self {
        Self { 
            resource_repository,
            resource_query_repo,
            category_repository,
            tag_respository,
            tag_query_repository,
        }
    }

    pub async fn create_resource(&self, data: CreateResourceDto) -> Result<ResourceID, ResourceError> {
        let command = CreateResourceCommand::from(data);

        let result = CreateResourceHandler::register(self.resource_repository, self.category_repository)
            .execute(command)
            .await
            .map_err(|err| ResourceError::Create(anyhow!(err)))?;

        Ok(result)
    }

    pub async fn update_resource(&self, data: UpdateResourceDto) -> Result<ResourceID, ResourceError> {
        let command = UpdateResourceCommand::from(data);

        let result = UpdateResourceHandler::register(self.resource_repository)
            .execute(command)
            .await
            .map_err(|err| ResourceError::Update(anyhow!(err)))?;

        Ok(result)
    }

    pub async fn add_resource_tag(&self, data: ResourceAddTagDto) -> Result<ResourceID, ResourceError> {
        let command = ResourceAddTagCommand::from(data);

        let result = ResourceAddTagHandler::register(self.resource_repository, self.tag_respository)
            .execute(command)
            .await
            .map_err(|err| ResourceError::AddTag(anyhow!(err)))?;

        Ok(result)
    }

    pub async fn remove_resource_tag(&self, data: ResourceRemoveTagDto) -> Result<ResourceID, ResourceError> {
        let command = ResourceRemoveTagCommand::from(data);

        let result = ResourceRemoveTagHandler::register(self.resource_repository, self.tag_respository)
            .execute(command)
            .await
            .map_err(|err| ResourceError::RemoveTag(anyhow!(err)))?;

        Ok(result)
    }



    pub async fn get_resource_by_id(&self, resource_id: String) -> Result<Option<ResourceResDto>, ResourceError> {
        let query = GetByIdResourceQuery { id: resource_id };

        let result = GetByIdResourceHandler::register(self.resource_query_repo)
            .query(query)
            .await
            .map_err(|err| ResourceError::GetById(anyhow!(err)))?;

        Ok(result)
    }

    pub async fn get_all_resource(&self) -> Result<Vec<ResourceResDto>, ResourceError> {
        let query = GetAllResourceQuery {};

        let result = GetAllResourceHandler::register(self.resource_query_repo)
            .query(query)
            .await
            .map_err(|err| ResourceError::GetAll(anyhow!(err)))?;

        Ok(result)
    }

    pub async fn resource_detail(&self, resource_id: String) -> Result<Option<ResourceDetailDto>, ResourceError> {
        let query = ResourceDetailQuery { id: resource_id };

        let result = ResourceDetailHandler::register(self.resource_query_repo)
            .query(query)
            .await
            .map_err(|err| ResourceError::Detail(anyhow!(err)))?;

        Ok(result)
    }

    pub async fn expore_the_file(&self, file_path: String) -> Result<(), ResourceError> {
        let path = Path::new(file_path.as_str());

        if path.exists() == false {
            return Err(ResourceError::ExploreFile(anyhow!(ResourceGenericError::FilePathNotExist())));
        }
        // TODO: For now, Windows Only 
        Command::new("explorer")
            .args(["/select,", file_path.as_str()]) // The comma after select is not a typo
            .spawn()
            .unwrap();

        Ok(())
    }

    pub async fn list_resource(&self, data: ResourceListQueryDto) -> Result<Vec<ResourceResDto>, ResourceError> {
        let query = ListResourceQuery::from(data);
        
        let result = ListResourceHandler::register(self.resource_query_repo)
            .query(query)
            .await
            .map_err(|err| ResourceError::Query(anyhow!(err)))?;

        Ok(result)
    }

    pub async fn querying_by_string(&self, query_string: String, belong_category: Option<String>) 
        -> Result<Vec<ResourceResDto>, ResourceError> 
    {
        let query = StringResourceQuery { query_string, belong_category };

        let result = StringResourceHandler::register(
                self.resource_query_repo,
                self.tag_query_repository,
            )
            .query(query)
            .await
            .map_err(|err| ResourceError::QueryingByString(anyhow!(err)))?;

        Ok(result)
    }
}