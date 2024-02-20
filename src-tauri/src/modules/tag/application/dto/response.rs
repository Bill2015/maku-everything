use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::modules::common::application::thing_serialize;

// Quick Example: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=15cfab66d38ff8a15a9cf1d8d897ac68
// See also: https://serde.rs/enum-representations.html
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "tag_type", content = "attr")]
#[serde(rename_all = "snake_case")]
pub enum TagAttrDto {
    Normal,

    Number { start: i64, end: i64, defval: i64 },

    Text { defval: String },

    Date { defval: String },

    Bool { defval: bool },
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
    pub attrval: TagAttrDto,

    pub created_at: String,

    pub updated_at: String,
}
