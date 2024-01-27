use serde::{Serialize, Deserialize};

use crate::{category::domain::CategoryID, tag::domain::TagID};

use super::ResourceID;

#[derive(Serialize, Deserialize)]
pub struct PortingResourceObject {
    pub id: ResourceID,

    pub name: String,

    pub description: String,

    pub belong_category: CategoryID,

    pub file: Option<String>,

    pub root_path: String,

    pub url: Option<String>,

    pub created_at: String,

    pub updated_at: String,

    pub tags: Vec<TagID>,

    pub auth: bool,
}
