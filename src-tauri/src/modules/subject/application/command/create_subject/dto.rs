use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CreateSubjectDto {
    pub name: String,

    pub description: String,

    pub belong_category: String,
}
