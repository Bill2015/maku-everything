use serde::{Deserialize, Serialize};

use crate::modules::tag::domain::TagID;

use super::CategoryID;

#[derive(Clone, Serialize, Deserialize)]
pub struct CategoryAddRulePlainObject {
    pub text: String,

    pub tag_id: TagID,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CategoryPlainObject {
    pub id: CategoryID,
    
    pub name: String,
    
    pub description: String,

    pub root_path: String,

    pub updated_at: String,

    pub rules: Vec<CategoryAddRulePlainObject>,

    pub created_at: String,

    pub auth: bool
}
