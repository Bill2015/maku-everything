use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct CreateSubjectDto {
    pub name: String,

    pub description: String,

    pub belong_category: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateSubjectDto {
    pub id: String,

    pub name: Option<String>,

    pub description: Option<String>,

    pub auth: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct QuerySubjectDto { 
    pub id: Option<String>,

    pub name: Option<String>,

    pub belong_category: Option<String>, 

    pub order_by: Option<String>,
    
    pub limit: Option<i64>,

    pub start: Option<i64>,
}
