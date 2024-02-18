use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::modules::tag::domain::TagID;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceTaggingAttrVO {
    Normal,
    Number(i64),
    Text(String),
    Date(DateTime<Utc>),
    Bool(bool),
}

#[derive(Debug, Clone, Serialize)]
pub struct ResourceTaggingVO {
    pub id: TagID,
    pub added_at: DateTime<Utc>,
    pub attrval: ResourceTaggingAttrVO,
}

impl ResourceTaggingVO {
    pub fn new<S: Into<String>>(tag_id: S, attrval: ResourceTaggingAttrVO) -> Self {
        Self {
            id: TagID::from(tag_id.into()),
            added_at: Utc::now(),
            attrval: attrval,
        }
    }
}
