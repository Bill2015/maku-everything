use serde::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize)]
pub struct UpdateCategoryAddRuleItemDto {
    pub text: String,
    pub tag_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateCategoryAddRuleDto {
    pub id: String,
    pub rules: Vec<UpdateCategoryAddRuleItemDto>
}

#[derive(Serialize, Deserialize)]
pub struct QueryCategoryDto { 
    pub id: Option<String>,

    pub name: Option<String>,

    pub order_by: Option<String>,
    
    pub limit: Option<i64>,

    pub start: Option<i64>,
}