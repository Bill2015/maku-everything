use surrealdb::sql::Thing;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceResDto {
    pub id: Thing,

    pub title: String,

    pub description: String,

    pub file_id: String,

    pub file_name: String,

    pub file_path: String,

    pub file_type: String,

    pub created_at: String,

    pub updated_at: String,
}
