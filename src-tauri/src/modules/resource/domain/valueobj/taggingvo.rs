use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::modules::tag::domain::TagID;

#[derive(Debug, Clone, Serialize)]
pub struct ResourceTaggingVO {
    pub id: TagID,
    pub added_at: DateTime<Utc>,
}

impl ResourceTaggingVO {
    pub fn new<S: Into<String>>(tag_id: S) -> Self {
        Self {
            id: TagID::from(tag_id.into()),
            added_at: Utc::now(),
        }
    }
}
