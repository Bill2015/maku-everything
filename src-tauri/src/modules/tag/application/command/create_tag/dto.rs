use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CreateTagDto {
    pub name: String,

    pub description: String,

    pub belong_category: String,

    pub belong_subject: String,
}
