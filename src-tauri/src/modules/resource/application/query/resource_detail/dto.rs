use surrealdb::sql::Thing;
use serde::{Deserialize, Serialize};

use crate::modules::common::application::thing_serialize;
use crate::modules::resource::application::dto::{ResourceFileDto, ResourceTaggingAttrPayloadDto, ResourceUrlDto};
use crate::modules::tag::application::dto::TagAttrDto;

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

    #[serde(flatten)]
    pub attr: TagAttrDto,

    pub attrval: ResourceTaggingAttrPayloadDto,

    pub added_at: String,

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
