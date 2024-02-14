use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ExportCategoryDto {
    pub id: String,
}
