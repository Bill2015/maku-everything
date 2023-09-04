use async_trait::async_trait;

use crate::category;
use crate::category::application::dto::CategoryError;
use crate::category::repository::CategoryRepository;
use crate::common::application::ICommandHandler;

pub struct UpdateCategoryCommand {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub auth: Option<bool>,
}

// =====================================
pub struct UpdateCategoryHandler<'a> {
    categroy_repo: &'a CategoryRepository<'a>,
}

impl<'a> UpdateCategoryHandler<'a> {
    pub fn register(categroy_repo: &'a CategoryRepository) -> Self {
        UpdateCategoryHandler { categroy_repo: &categroy_repo }
    }
}

#[async_trait]
impl ICommandHandler<UpdateCategoryCommand> for UpdateCategoryHandler<'_> {

    fn get_name() -> String {
        String::from("Change Category Command")
    }

    type Output = Result<String, CategoryError>;

    async fn execute(&self, command: UpdateCategoryCommand) -> Self::Output {
        let UpdateCategoryCommand { 
            id,
            name,
            description, 
            auth,
        } = command;

        // find by id
        let category_result = self.categroy_repo
            .find_by_id(&id)
            .await;

        let mut category = category_result
            .ok()
            .flatten()
            .ok_or_else(|| CategoryError::Update(id))?;
 
        // change name
        if name.is_some() {
            category.change_name(name.unwrap());
        }

        // change description
        if description.is_some() {
            category.change_description(description.unwrap());
        }

        // change auth
        if auth.is_some() {
            category.change_auth(auth.unwrap());
        }

        // save
        let reuslt = self.categroy_repo
            .save(category)
            .await;

        Ok(String::from("OK"))
    }
}