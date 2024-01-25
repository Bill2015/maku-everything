use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct ImportCategoryObjDto {
    pub id: String,
    pub description: String,
    pub name: String,
    pub updated_at: String,
    pub created_at: String,
    pub auth: bool
}

#[derive(Serialize, Deserialize)]
pub struct ImportCategoryOfSubjectObjDto {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
    pub auth: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ImportCategoryOfTagObjDto {
    pub id: String,
    pub name: String,
    pub description: String,
    pub belong_subject: String,
    pub created_at: String,
    pub updated_at: String,
    pub auth: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ImportCategoryOfResourceObjDto {
    pub id: String,
    pub name: String,
    pub description: String,
    pub file: Option<String>,
    pub url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub tags: Vec<String>,
    pub auth: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ImportCategoryDto {
    pub root_path: String,

    pub category: ImportCategoryObjDto,
    
    pub subjects: Vec<ImportCategoryOfSubjectObjDto>,

    pub tags: Vec<ImportCategoryOfTagObjDto>,

    pub resources: Vec<ImportCategoryOfResourceObjDto>,
    
    pub skip_when_resource_not_found: bool,
}