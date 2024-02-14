use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CreateResourceDto {
    pub name: String,

    pub description: String,

    pub belong_category: String,

    pub tags: Option<Vec<String>>,

    pub file_path: Option<String>,

    pub url_path: Option<String>,
}
