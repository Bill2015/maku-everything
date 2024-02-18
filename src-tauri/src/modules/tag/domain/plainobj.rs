use serde::{Deserialize, Serialize};

use crate::modules::category::domain::CategoryID;
use crate::modules::subject::domain::SubjectID;

use super::TagID;

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "tag_type", content = "attr")]
pub enum TagAttributePlainObject {
    #[serde(rename = "normal")]
    Normal,

    #[serde(rename = "number")]
    Number { start: i64, end: i64, defval: i64 },

    #[serde(rename = "text")]
    Text { defval: String },

    #[serde(rename = "date")]
    Date { defval: String },

    #[serde(rename = "bool")]
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
