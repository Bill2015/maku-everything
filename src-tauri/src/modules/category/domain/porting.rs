use serde::{Deserialize, Serialize};

use crate::modules::tag::domain::TagID;

use super::CategoryID;

#[derive(Serialize, Deserialize)]
pub struct PortingRuleItemObject {
    pub text: String,

    pub tag_id: TagID,
}

#[derive(Serialize, Deserialize)]
pub struct PortingCategoryObject {
    pub id: CategoryID,
    
    pub name: String,
    
    pub description: String,

    pub root_path: String,

    pub updated_at: String,

    pub rule_table: Vec<PortingRuleItemObject>,

    pub created_at: String,

    pub auth: bool
}
