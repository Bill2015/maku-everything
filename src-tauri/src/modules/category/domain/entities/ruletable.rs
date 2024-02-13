use std::collections::HashSet;

use serde::Serialize;

use crate::modules::category::domain::CategoryGenericError;
use crate::modules::category::domain::valueobj::CategoryAddRuleItemVO;
use crate::modules::tag::domain::TagID;

#[derive(Debug, Clone, Serialize)]
pub struct CategoryAddRuleEntity {
    rules: Vec<CategoryAddRuleItemVO>
}

impl CategoryAddRuleEntity  {
    pub fn new(rules: Vec<CategoryAddRuleItemVO>) -> Self {
        Self { rules }
    }

    pub fn get_rules(&self) -> &Vec<CategoryAddRuleItemVO> {
        &self.rules
    }

    pub fn take_rules(self) -> Vec<CategoryAddRuleItemVO> {
        self.rules
    }

    pub fn add_rule(&mut self, text: String, tag_id: TagID) -> Result<(), CategoryGenericError> {
        if let Some(_) = self.rules.iter().find(|x| x.text == text) {
            return Err(CategoryGenericError::DuplicatedRuleText());
        }
        self.rules.push(CategoryAddRuleItemVO::new(text, tag_id));
        Ok(())
    }

    pub fn update_rules(&mut self, rules: Vec<CategoryAddRuleItemVO>) -> Result<(), CategoryGenericError> {
        let mut set: HashSet<&String> = HashSet::new();
        for rule in &rules {
            if set.contains(&rule.text) {
                return Err(CategoryGenericError::DuplicatedRuleText());
            }
            set.insert(&rule.text);
        }

        self.rules = rules;
        Ok(())
    }
}
