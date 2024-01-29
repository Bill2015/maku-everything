use serde::{Serialize, Deserialize};
use crate::modules::category::domain::PortingCategoryObject;
use crate::modules::resource::domain::PortingResourceObject;
use crate::modules::subject::domain::PortingSubjectObject;
use crate::modules::tag::domain::PortingTagObject;

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

#[derive(Serialize, Deserialize)]
pub struct ImportCategoryDto {
    pub new_root_path: String,

    pub data: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExportCategoryDto {
    pub id: String,
}