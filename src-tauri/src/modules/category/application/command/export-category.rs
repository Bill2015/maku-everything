use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;

use crate::command_from_dto;
use crate::category::application::dto::{ExportCategoryDto, ExportCategoryResDto};
use crate::category::domain::CategoryGenericError;
use crate::category::repository::CategoryRepository;
use crate::common::application::ICommandHandler;
use crate::common::domain::Porting;
use crate::resource::domain::{PortingResourceObject, ResourceGenericError};
use crate::resource::infrastructure::ResourceQueryBuilder;
use crate::resource::repository::ResourceRepository;
use crate::subject::domain::{PortingSubjectObject, SubjectGenericError};
use crate::subject::infrastructure::SubjectQueryBuilder;
use crate::subject::repository::SubjectRepository;
use crate::tag::domain::{PortingTagObject, TagGenericError};
use crate::tag::infrastructure::TagQueryBuilder;
use crate::tag::repository::TagRepository;

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
            .export_to()?;

        // ------------------------------
        // subject part
        let subjects = self.subject_repo
            .get_by(
                SubjectQueryBuilder::new()
                    .set_belong_category(&category_id)
            )
            .await
            .or(Err(SubjectGenericError::DBInternalError()))?
            .into_iter()
            .map(|val| { val.export_to() })
            .collect::<Result<Vec<PortingSubjectObject>, SubjectGenericError>>()?;

        // ------------------------------
        // tag part
        let tags = self.tag_repo            
            .get_by(
                TagQueryBuilder::new()
                    .set_belong_category(&category_id)
            )
            .await
            .or(Err(TagGenericError::DBInternalError()))?
            .into_iter()
            .map(|val| { val.export_to() })
            .collect::<Result<Vec<PortingTagObject>, TagGenericError>>()?;

        // ------------------------------
        // resource part
        let resources = self.resource_repo
            .get_by(
                ResourceQueryBuilder::new()
                    .set_belong_category(&category_id)
            )
            .await
            .or(Err(ResourceGenericError::DBInternalError()))?
            .into_iter()
            .map(|val| { val.export_to() })
            .collect::<Result<Vec<PortingResourceObject>, ResourceGenericError>>()?;

        let data = serde_json::to_string::<ExportCategoryResDto>(&ExportCategoryResDto { 
            category,
            subjects, 
            tags, 
            resources 
        })?;

        Ok(data)
    }
}
