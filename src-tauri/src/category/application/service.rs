
use crate::category::repository::{CATEGORY_REPOSITORY, CATEGORY_QUERY_REPOSITORY, CategoryRepository, CategoryQueryRepository};
use crate::category::application::command::{CreateCategoryCommand, CreateCategoryHandler};
use crate::common::application::{ICommandHandler, IQueryHandler};
use crate::category::domain::CategoryError;

use super::command::{UpdateCategoryCommand, UpdateCategoryHandler};
use super::dto::CategoryResDto;
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
            category_repository: category_repository,
            category_queryrepo: category_queryrepo,
        }
    }

    pub async fn create_category(&self, name: String, description: String, root_path: String) -> Result<String, CategoryError> {
        let command = CreateCategoryCommand {
            name: name,
            root_path: root_path,
            description: description,
            auth: false,
        };
        let handler = CreateCategoryHandler::register(self.category_repository);
        
        let res = handler.execute(command).await?;

        Ok(res)
    }

    pub async fn update_category(&self, id: String, name: Option<String>, description: Option<String>, auth: Option<bool>) -> Result<String, CategoryError> {
        let command = UpdateCategoryCommand {
            id: id,
            name: name,
            description: description,
            auth: auth,
        };

        let handler = UpdateCategoryHandler::register(self.category_repository);

        let res = handler.execute(command).await?;

        Ok(res)
    }

    pub async fn get_all_category(&self) -> Result<Vec<CategoryResDto>, CategoryError> {
        let query = GetAllCategoryQuery { };

        let handler = GetAllCategoryHandler::register(self.category_queryrepo);

        let res = handler.query(query).await?;

        Ok(res)
    }

    pub async fn get_category_by_id(&self, id: String) -> Result<Option<CategoryResDto>, CategoryError> {
        let query = GetByIdCategoryQuery { id: id };
        
        let handler = GetByIdCategoryHandler::register(self.category_queryrepo);

        let res = handler.query(query).await?;

        Ok(res)
    }
}