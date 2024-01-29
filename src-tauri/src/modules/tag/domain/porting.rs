use serde::{Deserialize, Serialize};

use crate::category::domain::CategoryID;
use crate::subject::domain::SubjectID;

use super::TagID;

#[derive(Serialize, Deserialize)]
pub struct PortingTagObject {
    pub id: TagID,

    pub name: String,

    pub description: String,

    pub belong_category: CategoryID,

    pub belong_subject: SubjectID,

    pub created_at: String,

    pub updated_at: String,

    pub auth: bool,
}
