use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::modules::common::application::thing_serialize;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "tag_type", content = "attr")]
pub enum TagAttrResDto {
    #[serde(rename = "normal")]
    Normal,

    #[serde(rename = "number")]
    WithNumber {
        start: i64,
        end: i64,
        defval: i64,
    },

    #[serde(rename = "text")]
    WithText { defval: String },

    #[serde(rename = "date")]
    WithDate { defval: String },

    #[serde(rename = "bool")]
    WithBool { defval: bool },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TagResDto {
    #[serde(serialize_with = "thing_serialize")]
    pub id: Thing,

    pub name: String,

    pub description: String,

    #[serde(serialize_with = "thing_serialize")]
    pub belong_category: Thing,

    pub category_name: String,

    pub tagged_count: i64,

    #[serde(serialize_with = "thing_serialize")]
    pub belong_subject: Thing,

    pub subject_name: String,

    pub auth: bool,

    #[serde(flatten)]
    pub attrval: TagAttrResDto,

    pub created_at: String,

    pub updated_at: String,
}
