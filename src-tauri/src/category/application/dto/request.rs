use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct CreateCategoryDto {
    pub name: String,

    pub description: String,
    
    pub root_path: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateCategoryDto {
    pub id: String,

    pub name: Option<String>,

    pub description: Option<String>,

    pub auth: Option<bool>,
}
