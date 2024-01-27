use serde::{Serialize, Deserialize};
use crate::category::domain::PortingCategoryObject;
use crate::resource::domain::PortingResourceObject;
use crate::subject::domain::PortingSubjectObject;
use crate::tag::domain::PortingTagObject;

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

    pub category: PortingCategoryObject,
    
    pub subjects: Vec<PortingSubjectObject>,

    pub tags: Vec<PortingTagObject>,

    pub resources: Vec<PortingResourceObject>,
}

#[derive(Serialize, Deserialize)]
pub struct ExportCategoryDto {
    pub id: String,
}