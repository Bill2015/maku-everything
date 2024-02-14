use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateCategoryDto {
    pub name: String,

    pub description: String,
    
    pub root_path: String,
}
