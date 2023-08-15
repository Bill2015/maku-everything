use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize)]
pub struct SubjectResDto {
    pub id: Thing,

    pub name: String,

    pub description: String,

    pub auth: bool,

    pub created_at: String,

    pub updated_at: String,
}
