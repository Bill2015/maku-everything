use serde::{Serialize, Deserialize};

use crate::modules::category::domain::CategoryID;

use super::SubjectID;

#[derive(Serialize, Deserialize)]
pub struct PortingSubjectObject {
    pub id: SubjectID,

    pub name: String,

    pub description: String,

    pub belong_category: CategoryID,

    pub created_at: String,

    pub updated_at: String,

    pub auth: bool,
}
