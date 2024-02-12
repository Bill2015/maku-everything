use serde::Serialize;

use crate::modules::tag::domain::TagID;

#[derive(Debug, Clone, Serialize)]
pub struct RuleItemVO {
    pub text: String,
    pub tag_id: TagID,
}
impl RuleItemVO {
    pub fn new(text: String, tag_id: TagID) -> Self {
        Self { text, tag_id }
    }
}
