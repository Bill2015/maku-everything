use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CreateResourceDto {
    pub name: String,

    pub description: String,

    pub belong_category: String,

    pub file_path: String,

    pub url_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateResourceDto {
    pub id: String,

    pub name: Option<String>,

    pub description: Option<String>,

    pub auth: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct ResourceAddTagDto {
    pub id: String,

    pub tag_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResourceRemoveTagDto {
    pub id: String,

    pub tag_id: String,
}