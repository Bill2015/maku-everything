use surrealdb::sql::Thing;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceFileDto {
    pub uuid: String,

    pub name: String,

    pub path: String,

    pub ext: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceResDto {
    pub id: Thing,

    pub title: String,

    pub description: String,

    pub file: ResourceFileDto,

    pub created_at: String,

    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceTagDto {
    pub id: Thing,

    pub name: String,

    pub description: String,

    pub belong_subject: Thing,

    pub subject_name: String,

    pub created_at: String,

    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceDetailDto {
    pub id: Thing,

    pub title: String,

    pub description: String,

    pub file: ResourceFileDto,

    pub belong_category: Thing,

    pub created_at: String,

    pub updated_at: String,

    pub tags: Vec<ResourceTagDto>,
}
