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
