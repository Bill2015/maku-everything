use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize)]
pub struct TagResDto {
    pub id: Thing,

    pub name: String,

    pub description: String,

    pub belong_category: Thing,

    pub category_name: String,

    pub tag_nums: i64,

    pub belong_subject: Thing,

    pub subject_name: String,

    pub auth: bool,

    pub created_at: String,

    pub updated_at: String,
}
