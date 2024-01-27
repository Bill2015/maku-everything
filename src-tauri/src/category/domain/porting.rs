use serde::{Deserialize, Serialize};

use super::CategoryID;

#[derive(Serialize, Deserialize)]
pub struct PortingCategoryObject {
    pub id: CategoryID,
    
    pub name: String,
    
    pub description: String,

    pub root_path: String,

    pub updated_at: String,

    pub created_at: String,

    pub auth: bool
}
