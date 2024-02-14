use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UpdateCategoryDto {
    pub id: String,

    pub name: Option<String>,

    pub description: Option<String>,

    pub auth: Option<bool>,
}
