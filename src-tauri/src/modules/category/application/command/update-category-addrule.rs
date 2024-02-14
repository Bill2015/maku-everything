use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;

use crate::modules::category::application::dto::{UpdateCategoryMapperRuleDto, UpdateCategoryMapperRuleItemDto};
use crate::modules::category::domain::{CategoryMapperRuleItemVO, CategoryGenericError, CategoryID};
use crate::modules::category::repository::CategoryRepository;
use crate::modules::common::application::ICommandHandler;
use crate::command_from_dto;
use crate::modules::tag::repository::TagRepository;

#[derive(Deserialize)]
pub struct UpdateCategoryMapperRuleCommand {
    pub id: String,
    pub rules: Vec<UpdateCategoryMapperRuleItemDto>
}
command_from_dto!(UpdateCategoryMapperRuleCommand, UpdateCategoryMapperRuleDto);

// =====================================
pub struct UpdateCategoryMapperRuleHandler<'a> {
    categroy_repo: &'a CategoryRepository<'a>,
    tag_repo: &'a TagRepository<'a>,
}

impl<'a> UpdateCategoryMapperRuleHandler<'a> {
    pub fn register(categroy_repo: &'a CategoryRepository, tag_repo: &'a TagRepository) -> Self {
        Self { categroy_repo: &categroy_repo, tag_repo: &tag_repo }
    }
}

#[async_trait]
impl ICommandHandler<UpdateCategoryMapperRuleCommand> for UpdateCategoryMapperRuleHandler<'_> {

    fn get_name() -> String {
        String::from("Change Category Command")
    }

    type Output = CategoryID;

    async fn execute(&self, command: UpdateCategoryMapperRuleCommand) -> Result<Self::Output, Error> {
        let UpdateCategoryMapperRuleCommand { id, rules } = command;

        // check tag id
        for rule in &rules {
            if self.tag_repo.is_exist(&rule.tag_id).await == false {
                return Err(CategoryGenericError::IdNotFounded())?;
            }
        }
        
        let rule_items = rules
            .into_iter()
            .map(|val| CategoryMapperRuleItemVO::from(val.text, val.tag_id))
            .collect::<Vec<CategoryMapperRuleItemVO>>();

        // find by id
        let mut category = self.categroy_repo
            .find_by_id(&id)
            .await
            .or(Err(CategoryGenericError::DBInternalError()))?
            .ok_or(CategoryGenericError::IdNotFounded())?;

        category
            .get_mut_rule_table()
            .update_rules(rule_items)?;

        // save
        let result = self.categroy_repo
            .save(category)
            .await;

        match result {
            Ok(value) => Ok(value.take_id()),
            _ => Err(CategoryGenericError::DBInternalError().into()),
        }
    }
}