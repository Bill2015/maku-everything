use anyhow::Error;
use async_trait::async_trait;

use crate::modules::common::application::IQueryHandler;
use crate::modules::category::repository::CategoryQueryRepository;
use crate::modules::category::domain::CategoryGenericError;

mod dto;
pub use dto::*;

pub struct GetMapperRulesCategoryQuery { 
    pub id: String,
}

// =====================================
pub struct GetMapperRulesCategoryHandler<'a> {
    category_repo: &'a CategoryQueryRepository<'a>,
}

impl<'a> GetMapperRulesCategoryHandler<'a> {
    pub fn register(category_repo: &'a CategoryQueryRepository) -> Self {
        GetMapperRulesCategoryHandler { category_repo: &category_repo }
    }
}

#[async_trait]
impl IQueryHandler<GetMapperRulesCategoryQuery> for GetMapperRulesCategoryHandler<'_>{
    fn get_name() -> String {
        String::from("Get Category add rules")
    }

    type Output = Option<CategoryMapperRulesResDto>;

    async fn query(&self, query: GetMapperRulesCategoryQuery) -> Result<Self::Output, Error> {
        let GetMapperRulesCategoryQuery { id } = query;

        let result = self.category_repo
            .get_mapper_rules(&id)
            .await;
    
        match result {
            Ok(value) => Ok(value),
            _ => Err(CategoryGenericError::DBInternalError().into()),
        }
    }
}
