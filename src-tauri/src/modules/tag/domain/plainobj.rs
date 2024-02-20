use serde::{Deserialize, Serialize};

use crate::modules::category::domain::CategoryID;
use crate::modules::subject::domain::SubjectID;

use super::TagID;

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "tag_type", content = "attr")]
#[serde(rename_all = "snake_case")]
pub enum TagAttributePlainObject {
    Normal,

    Number { start: i64, end: i64, defval: i64 },

    Text { defval: String },

    Date { defval: String },

    Bool { defval: bool },
}


#[derive(Clone, Serialize, Deserialize)]
pub struct TagPlainObject {
    pub id: TagID,

    pub name: String,

    pub description: String,

    pub belong_category: CategoryID,

    pub belong_subject: SubjectID,

    pub created_at: String,

    pub updated_at: String,

    pub auth: bool,
    
    #[serde(flatten)]
    pub attrval: TagAttributePlainObject,
}
