use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize)]
pub struct TagResDto {
    pub id: Thing,

    pub name: String,

    pub description: String,

    pub belong_category: String,

    pub belong_subject: String,

    pub auth: bool,

    pub created_at: String,

    pub updated_at: String,
}
