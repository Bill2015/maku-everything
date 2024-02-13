use serde::Serialize;

use crate::modules::category::domain::CategoryGenericError;
use crate::modules::category::domain::valueobj::RuleItemVO;
use crate::modules::tag::domain::TagID;

#[derive(Debug, Clone, Serialize)]
pub struct RuleTableEntity {
    rules: Vec<RuleItemVO>
}

impl RuleTableEntity  {
    pub fn new(rules: Vec<RuleItemVO>) -> Self {
        Self { rules }
    }

    pub fn get_rules(&self) -> &Vec<RuleItemVO> {
        &self.rules
    }

    pub fn take_rules(self) -> Vec<RuleItemVO> {
        self.rules
    }

    pub fn add_rule(&mut self, text: String, tag_id: TagID) -> Result<(), CategoryGenericError> {
        if let Some(_) = self.rules.iter().find(|x| x.text == text) {
            return Err(CategoryGenericError::DuplicatedRuleText());
        }
        self.rules.push(RuleItemVO::new(text, tag_id));
        Ok(())
    }
}
