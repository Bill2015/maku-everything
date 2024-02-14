use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct QueryCategoryDto { 
    pub id: Option<String>,

    pub name: Option<String>,

    pub order_by: Option<String>,
    
    pub limit: Option<i64>,

    pub start: Option<i64>,
}
