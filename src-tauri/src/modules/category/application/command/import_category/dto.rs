use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ImportCategoryDto {
    pub new_root_path: String,

    pub data: String,
}
