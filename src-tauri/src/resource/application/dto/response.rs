use surrealdb::sql::Thing;
use serde::{Deserialize, Serialize};

use crate::common::application::thing_serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceUrlDto {
    pub full: String,

    pub host: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceFileDto {
    pub uuid: String,

    pub name: String,

    pub path: String,

    pub ext: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceResDto {
    #[serde(serialize_with = "thing_serialize")]
    pub id: Thing,

    pub name: String,

    pub description: String,

    pub root_path: String,

    pub file: Option<ResourceFileDto>,

    pub url: Option<ResourceUrlDto>,

    pub created_at: String,

    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceTagDto {
    #[serde(serialize_with = "thing_serialize")]
    pub id: Thing,

    pub name: String,

    pub description: String,

    #[serde(serialize_with = "thing_serialize")]
    pub belong_subject: Thing,

    pub subject_name: String,

    pub tagged_count: i64,

    pub created_at: String,

    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceDetailDto {
    #[serde(serialize_with = "thing_serialize")]
    pub id: Thing,

    pub name: String,

    pub description: String,

    pub root_path: String,

    pub file: Option<ResourceFileDto>,

    pub url: Option<ResourceUrlDto>,

    #[serde(serialize_with = "thing_serialize")]
    pub belong_category: Thing,

    pub created_at: String,

    pub updated_at: String,

    pub tags: Vec<ResourceTagDto>,
}
