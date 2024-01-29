use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct CreateSubjectDto {
    pub name: String,

    pub description: String,

    pub belong_category: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateSubjectDto {
    pub id: String,

    pub name: Option<String>,

    pub description: Option<String>,

    pub auth: Option<bool>,
}