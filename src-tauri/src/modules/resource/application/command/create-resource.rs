use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;

use crate::command_from_dto;
use crate::modules::category::repository::CategoryRepository;
use crate::modules::resource::application::dto::CreateResourceDto;
use crate::modules::resource::domain::{ResourceAggregate, ResourceGenericError, ResourceID};
use crate::modules::resource::repository::ResourceRepository;
use crate::modules::common::application::ICommandHandler;
use crate::modules::tag::domain::TagID;
use crate::modules::tag::repository::TagRepository;

#[derive(Deserialize)]
pub struct CreateResourceCommand {
    pub name: String,

    pub description: String,

    pub belong_category: String,

    pub tags: Option<Vec<String>>,

    pub file_path: Option<String>,

    pub url_path: Option<String>,
}
command_from_dto!(CreateResourceCommand, CreateResourceDto);

// =====================================
pub struct CreateResourceHandler<'a> {
    resource_repo: &'a ResourceRepository<'a>,
    category_repo: &'a CategoryRepository<'a>,
    tag_repo: &'a TagRepository<'a>,
}

impl<'a> CreateResourceHandler<'a> {
    pub fn register(
        resource_repo: &'a ResourceRepository<'a>, 
        category_repo: &'a CategoryRepository<'a>,
        tag_repo: &'a TagRepository<'a>
    ) -> Self {
        Self { resource_repo, category_repo, tag_repo }
    }
}

#[async_trait]
impl ICommandHandler<CreateResourceCommand> for CreateResourceHandler<'_> {

    fn get_name() -> String {
        String::from("Create Resource Command")
    }

    type Output = ResourceID;

    async fn execute(&self, command: CreateResourceCommand) -> Result<Self::Output, Error> {
        let CreateResourceCommand { 
            name,
            description,
            belong_category,
            tags,
            file_path,
            url_path,
        } = command;

        let category = self.category_repo
            .find_by_id(&belong_category)
            .await
            .or(Err(ResourceGenericError::DBInternalError()))?
            .ok_or(ResourceGenericError::BelongCategoryNotExists())?;


        // create new resource
        let mut new_resource = ResourceAggregate::new(
            name,
            description,
            &category.id,
            category.root_path,
            file_path,
            url_path
        )?;

        if let Some(tags) = tags {
            for tag in tags {
                let tag_id = self.tag_repo
                    .is_exist(&tag)
                    .await
                    .then(|| TagID::from(tag))
                    .ok_or(ResourceGenericError::TagNotExists())?;

                new_resource.get_mut_tagging().add_tag(&tag_id)?;
            }
        } 
        
        // save
        let result = self.resource_repo
            .save(new_resource)
            .await;
        
        match result {
            Ok(value) => Ok(value.id),
            _ => Err(ResourceGenericError::DBInternalError().into()),
        }
    }
}
