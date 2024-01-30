use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CreateResourceDto {
    pub name: String,

    pub description: String,

    pub belong_category: String,

    pub file_path: Option<String>,

    pub url_path: Option<String>,
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

#[derive(Serialize, Deserialize)]
pub struct ResourceListQueryDto {
    pub id: Option<String>,

    pub name: Option<String>,

    pub belong_category: Option<String>, 

    pub order_by: Option<String>,
}
