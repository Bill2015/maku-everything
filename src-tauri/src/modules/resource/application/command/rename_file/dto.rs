use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ResourceRenameFileDto {
    pub id: String,

    pub new_name: Option<String>,
}