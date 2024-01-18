
use crate::category::repository::{CATEGORY_REPOSITORY, CATEGORY_QUERY_REPOSITORY, CategoryRepository, CategoryQueryRepository};
use crate::category::application::command::{CreateCategoryCommand, CreateCategoryHandler};
use crate::common::application::{ICommandHandler, IQueryHandler};
use crate::category::domain::CategoryError;

use super::command::{UpdateCategoryCommand, UpdateCategoryHandler};
use super::dto::{CategoryResDto, CreateCategoryDto, UpdateCategoryDto};
use super::query::*;

pub static CATEGORY_SERVICE: CategoryService = CategoryService::init(&CATEGORY_REPOSITORY, &CATEGORY_QUERY_REPOSITORY);

pub struct CategoryService<'a> {
    category_repository: &'a CategoryRepository<'a>,
    category_queryrepo: &'a CategoryQueryRepository<'a>,
}
impl<'a> CategoryService<'a> {
    const fn init(
        category_repository: &'a CategoryRepository<'_>,
        category_queryrepo: &'a CategoryQueryRepository<'a>
    ) -> Self {
        CategoryService { 
            category_repository,
            category_queryrepo,
        }
    }

    pub async fn create(&self, data: CreateCategoryDto) -> Result<String, CategoryError> {
        let command = CreateCategoryCommand::from(data);

        let result = CreateCategoryHandler::register(self.category_repository)
            .execute(command)
            .await?;
        
        Ok(result)
    }

    pub async fn update(&self, data: UpdateCategoryDto) -> Result<String, CategoryError> {
        let command  = UpdateCategoryCommand::from(data);

        let result = UpdateCategoryHandler::register(self.category_repository)
            .execute(command)
            .await?;

        Ok(result)
    }

    pub async fn get_all(&self) -> Result<Vec<CategoryResDto>, CategoryError> {
        let query = GetAllCategoryQuery { };

        let result = GetAllCategoryHandler::register(self.category_queryrepo)
            .query(query)
            .await?;

        Ok(result)
    }

    pub async fn get_by_id(&self, id: String) -> Result<Option<CategoryResDto>, CategoryError> {
        let query = GetByIdCategoryQuery { id: id };
        
        let result = GetByIdCategoryHandler::register(self.category_queryrepo)
            .query(query)
            .await?;

        Ok(result)
    }
}