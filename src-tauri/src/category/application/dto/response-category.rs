use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize)]
pub struct CategoryResDto {
    pub id: Thing,

    pub title: String,

    pub resource_num: i64,

    pub description: String,

    pub auth: bool,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}
