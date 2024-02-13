use anyhow::Error;
use async_trait::async_trait;

use crate::modules::common::application::IQueryHandler;
use crate::modules::category::repository::CategoryQueryRepository;
use crate::modules::category::application::dto::CategoryAddRulesResDto;
use crate::modules::category::domain::CategoryGenericError;

pub struct GetAddRulesCategoryQuery { 
    pub id: String,
}

// =====================================
pub struct GetAddRulesCategoryHandler<'a> {
    category_repo: &'a CategoryQueryRepository<'a>,
}

impl<'a> GetAddRulesCategoryHandler<'a> {
    pub fn register(category_repo: &'a CategoryQueryRepository) -> Self {
        GetAddRulesCategoryHandler { category_repo: &category_repo }
    }
}

#[async_trait]
impl IQueryHandler<GetAddRulesCategoryQuery> for GetAddRulesCategoryHandler<'_>{
    fn get_name() -> String {
        String::from("Get Category add rules")
    }

    type Output = Option<CategoryAddRulesResDto>;

    async fn query(&self, query: GetAddRulesCategoryQuery) -> Result<Self::Output, Error> {
        let GetAddRulesCategoryQuery { id } = query;

        let result = self.category_repo
            .get_rules(&id)
            .await;
    
        match result {
            Ok(value) => Ok(value),
            _ => Err(CategoryGenericError::DBInternalError().into()),
        }
    }
}
