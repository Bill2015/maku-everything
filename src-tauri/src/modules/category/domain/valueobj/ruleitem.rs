use serde::Serialize;

use crate::modules::tag::domain::TagID;

#[derive(Debug, Clone, Serialize)]
pub struct CategoryMapperRuleItemVO {
    pub text: String,
    pub tag_id: TagID,
}

impl CategoryMapperRuleItemVO {
    pub fn new(text: String, tag_id: TagID) -> Self {
        Self { text, tag_id }
    }
    pub fn from(text: String, tag_id: String) -> Self {
        Self { text, tag_id: TagID::from(tag_id) }
    }
}
