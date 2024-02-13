use serde::{Serialize, Deserialize};

use crate::modules::{category::domain::CategoryID, tag::domain::TagID};

use super::ResourceID;


#[derive(Clone, Serialize, Deserialize)]
pub struct ResourceTaggingPlainObject {
    pub id: TagID,
    pub added_at: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ResourcePlainObject {
    pub id: ResourceID,
    pub name: String,
    pub description: String,
    pub belong_category: CategoryID,
    pub file: Option<String>,
    pub root_path: String,
    pub url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub tags: Vec<ResourceTaggingPlainObject>,
    pub auth: bool,
}
