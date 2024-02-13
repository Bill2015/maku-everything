use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;

use crate::command_from_dto;
use crate::modules::category::application::dto::{ExportCategoryDto, ExportCategoryResDto};
use crate::modules::category::domain::CategoryGenericError;
use crate::modules::category::repository::CategoryRepository;
use crate::modules::common::application::ICommandHandler;
use crate::modules::common::domain::ToPlainObject;
use crate::modules::common::infrastructure::QueryBuilder;
use crate::modules::resource::domain::{ResourcePlainObject, ResourceGenericError};
use crate::modules::resource::infrastructure::ResourceQueryBuilder;
use crate::modules::resource::repository::ResourceRepository;
use crate::modules::subject::domain::{SubjectPlainObject, SubjectGenericError};
use crate::modules::subject::infrastructure::SubjectQueryBuilder;
use crate::modules::subject::repository::SubjectRepository;
use crate::modules::tag::domain::{TagPlainObject, TagGenericError};
use crate::modules::tag::infrastructure::TagQueryBuilder;
use crate::modules::tag::repository::TagRepository;

#[derive(Deserialize)]
pub struct ExportCategoryCommand {
    id: String,
}
command_from_dto!(ExportCategoryCommand, ExportCategoryDto);

// =====================================
pub struct ExportCategoryHandler<'a> {
    categroy_repo: &'a CategoryRepository<'a>,
    subject_repo: &'a SubjectRepository<'a>,
    tag_repo: &'a TagRepository<'a>,
    resource_repo: &'a ResourceRepository<'a>,
}

impl<'a> ExportCategoryHandler<'a> {
    pub fn register(
        categroy_repo: &'a CategoryRepository,
        subject_repo: &'a SubjectRepository,
        tag_repo: &'a TagRepository,
        resource_repo: &'a ResourceRepository,
    ) -> Self {
        Self {
            categroy_repo,
            subject_repo,
            tag_repo,
            resource_repo,
        }
    }
}

#[async_trait]
impl ICommandHandler<ExportCategoryCommand> for ExportCategoryHandler<'_> {

    fn get_name() -> String {
        String::from("Create Category Command")
    }

    type Output = String;

    async fn execute(&self, command: ExportCategoryCommand) -> Result<Self::Output, Error> {
        let ExportCategoryCommand { id: category_id } = command;

        // save category
        let category = self.categroy_repo
            .find_by_id(&category_id)
            .await
            .or(Err(CategoryGenericError::DBInternalError()))?
            .ok_or(CategoryGenericError::IdNotFounded())?
            .to_plain();

        // ------------------------------
        // subject part
        let subjects = self.subject_repo
            .get_by(
                SubjectQueryBuilder::new()
                    .set_belong_category(&category_id)
                    .build()?
            )
            .await
            .or(Err(SubjectGenericError::DBInternalError()))?
            .into_iter()
            .map(|val| val.to_plain())
            .collect::<Vec<SubjectPlainObject>>();

        // ------------------------------
        // tag part
        let tags = self.tag_repo            
            .get_by(
                TagQueryBuilder::new()
                    .set_belong_category(&category_id)
                    .build()?
            )
            .await
            .or(Err(TagGenericError::DBInternalError()))?
            .into_iter()
            .map(|val| val.to_plain())
            .collect::<Vec<TagPlainObject>>();

        // ------------------------------
        // resource part
        let resources = self.resource_repo
            .get_by(
                ResourceQueryBuilder::new()
                    .set_belong_category(&category_id)
                    .build()?
            )
            .await
            .or(Err(ResourceGenericError::DBInternalError()))?
            .into_iter()
            .map(|val| val.to_plain())
            .collect::<Vec<ResourcePlainObject>>();

        let data = serde_json::to_string::<ExportCategoryResDto>(&ExportCategoryResDto { 
            category,
            subjects, 
            tags, 
            resources 
        })?;

        Ok(data)
    }
}
