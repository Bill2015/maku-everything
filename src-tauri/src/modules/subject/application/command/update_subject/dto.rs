use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UpdateSubjectDto {
    pub id: String,

    pub name: Option<String>,

    pub description: Option<String>,

    pub auth: Option<bool>,
}
