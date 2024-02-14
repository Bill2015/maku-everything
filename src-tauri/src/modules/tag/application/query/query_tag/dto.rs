use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct QueryTagDto { 
    pub id: Option<String>,

    pub name: Option<String>,

    pub belong_category: Option<String>, 

    pub belong_subject: Option<String>,

    pub belong_subject_name: Option<String>,

    pub tagging_resource: Option<String>,

    pub order_by: Option<String>,

    pub limit: Option<i64>,

    pub start: Option<i64>,
}
