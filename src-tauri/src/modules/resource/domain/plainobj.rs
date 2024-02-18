use serde::{Serialize, Deserialize};

use crate::modules::{category::domain::CategoryID, tag::domain::TagID};

use super::ResourceID;


#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "tagging_type", content = "attr")]
pub enum ResourceTaggingAttrPlainObject {
    #[serde(rename = "normal")]
    Normal,

    #[serde(rename = "number")]
    Number(i64),

    #[serde(rename = "text")]
    Text(String),

    #[serde(rename = "date")]
    Date(String),

    #[serde(rename = "bool")]
    Bool(bool),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ResourceTaggingPlainObject {
    pub id: TagID,

    pub added_at: String,

    #[serde(flatten)]
    pub attrval: ResourceTaggingAttrPlainObject,
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
