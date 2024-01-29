use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CreateTagDto {
    pub name: String,

    pub description: String,

    pub belong_category: String,

    pub belong_subject: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTagDto {
    pub id: String,

    pub name: Option<String>,

    pub description: Option<String>,

    pub auth: Option<bool>,
}