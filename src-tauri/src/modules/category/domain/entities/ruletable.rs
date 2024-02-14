use std::collections::HashSet;

use serde::Serialize;

use crate::modules::category::domain::CategoryGenericError;
use crate::modules::category::domain::valueobj::CategoryMapperRuleItemVO;
use crate::modules::tag::domain::TagID;

#[derive(Debug, Clone, Serialize)]
pub struct CategoryMapperRuleEntity {
    rules: Vec<CategoryMapperRuleItemVO>
}

impl CategoryMapperRuleEntity  {
    pub fn new(rules: Vec<CategoryMapperRuleItemVO>) -> Self {
        Self { rules }
    }

    pub fn get_rules(&self) -> &Vec<CategoryMapperRuleItemVO> {
        &self.rules
    }

    pub fn take_rules(self) -> Vec<CategoryMapperRuleItemVO> {
        self.rules
    }

    pub fn add_rule(&mut self, text: String, tag_id: TagID) -> Result<(), CategoryGenericError> {
        if let Some(_) = self.rules.iter().find(|x| x.text == text) {
            return Err(CategoryGenericError::DuplicatedMapperRuleText());
        }
        self.rules.push(CategoryMapperRuleItemVO::new(text, tag_id));
        Ok(())
    }

    pub fn update_rules(&mut self, rules: Vec<CategoryMapperRuleItemVO>) -> Result<(), CategoryGenericError> {
        let mut set: HashSet<&String> = HashSet::new();
        for rule in &rules {
            if set.contains(&rule.text) {
                return Err(CategoryGenericError::DuplicatedMapperRuleText());
            }
            set.insert(&rule.text);
        }

        self.rules = rules;
        Ok(())
    }
}
