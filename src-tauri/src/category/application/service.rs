
use anyhow::anyhow;

use crate::category::repository::{CATEGORY_REPOSITORY, CATEGORY_QUERY_REPOSITORY, CategoryRepository, CategoryQueryRepository};
use crate::category::application::command::{CreateCategoryCommand, CreateCategoryHandler};
use crate::common::application::{ICommandHandler, IQueryHandler};
use crate::category::domain::{CategoryError, CategoryID};
use crate::resource::repository::{ResourceRepository, RESOURCE_REPOSITORY};
use crate::subject::repository::{SubjectRepository, SUBJECT_REPOSITORY};
use crate::tag::repository::{TagRepository, TAG_REPOSITORY};

use super::command::{ExportCategoryCommand, ExportCategoryHandler, ImportCategoryCommand, ImportCategoryHandler, UpdateCategoryCommand, UpdateCategoryHandler};
use super::dto::*;
use super::query::*;

pub static CATEGORY_SERVICE: CategoryService = CategoryService::init(
    &CATEGORY_REPOSITORY,
    &SUBJECT_REPOSITORY,
    &TAG_REPOSITORY,
    &RESOURCE_REPOSITORY,
    &CATEGORY_QUERY_REPOSITORY
);

pub struct CategoryService<'a> {
    category_repository: &'a CategoryRepository<'a>,
    subject_repository: &'a SubjectRepository<'a>,
    tag_repository: &'a TagRepository<'a>,
    resource_repository: &'a ResourceRepository<'a>,
    category_queryrepo: &'a CategoryQueryRepository<'a>,
}
impl<'a> CategoryService<'a> {
    const fn init(
        category_repository: &'a CategoryRepository<'_>,
        subject_repository: &'a SubjectRepository<'a>,
        tag_repository: &'a TagRepository<'a>,
        resource_repository: &'a ResourceRepository<'a>,
        category_queryrepo: &'a CategoryQueryRepository<'a>
    ) -> Self {
        Self { 
            category_repository,
            subject_repository,
            tag_repository,
            resource_repository,
            category_queryrepo,
        }
    }

    pub async fn create(&self, data: CreateCategoryDto) -> Result<CategoryID, CategoryError> {
        let command = CreateCategoryCommand::from(data);

        let result = CreateCategoryHandler::register(self.category_repository)
            .execute(command)
            .await
            .map_err(|err| CategoryError::Create(anyhow!(err)))?;
        
        Ok(result)
    }

    pub async fn update(&self, data: UpdateCategoryDto) -> Result<CategoryID, CategoryError> {
        let command  = UpdateCategoryCommand::from(data);

        let result = UpdateCategoryHandler::register(self.category_repository)
            .execute(command)
            .await
            .map_err(|err| CategoryError::Update(anyhow!(err)))?;

        Ok(result)
    }

    pub async fn import(&self, data: ImportCategoryDto) -> Result<CategoryID, CategoryError> {
        let command = ImportCategoryCommand::from(data);

        let result = ImportCategoryHandler::register(
                self.category_repository,
                self.subject_repository,
                self.tag_repository,
                self.resource_repository,
            )
            .execute(command)
            .await
            .map_err(|err| CategoryError::Import(anyhow!(err)))?;

        Ok(result)
    }

    pub async fn export(&self, data: ExportCategoryDto) -> Result<ExportCategoryResDto, CategoryError> {
        let command = ExportCategoryCommand::from(data);

        let result = ExportCategoryHandler::register(
                self.category_repository,
                self.subject_repository,
                self.tag_repository,
                self.resource_repository,
            )
            .execute(command)
            .await
            .map_err(|err| CategoryError::Export(anyhow!(err)))?;

        Ok(result)
    }

    pub async fn get_all(&self) -> Result<Vec<CategoryResDto>, CategoryError> {
        let query = GetAllCategoryQuery { };

        let result = GetAllCategoryHandler::register(self.category_queryrepo)
            .query(query)
            .await
            .map_err(|err| CategoryError::GetAll(anyhow!(err)))?;

        Ok(result)
    }

    pub async fn get_by_id(&self, id: String) -> Result<Option<CategoryResDto>, CategoryError> {
        let query = GetByIdCategoryQuery { id: id };
        
        let result = GetByIdCategoryHandler::register(self.category_queryrepo)
            .query(query)
            .await
            .map_err(|err| CategoryError::GetById(anyhow!(err)))?;

        Ok(result)
    }
}