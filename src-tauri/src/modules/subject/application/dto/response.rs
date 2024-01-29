use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::common::application::thing_serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct SubjectResDto {
    #[serde(serialize_with = "thing_serialize")]
    pub id: Thing,

    pub name: String,

    pub description: String,

    #[serde(serialize_with = "thing_serialize")]
    pub belong_category: Thing,

    pub auth: bool,

    pub created_at: String,

    pub updated_at: String,
}
