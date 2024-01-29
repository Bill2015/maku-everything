use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::category::domain::PortingCategoryObject;
use crate::subject::domain::PortingSubjectObject;
use crate::tag::domain::PortingTagObject;
use crate::resource::domain::PortingResourceObject;
use crate::common::application::thing_serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct CategoryResDto {
    #[serde(serialize_with = "thing_serialize")]
    pub id: Thing,

    pub name: String,

    pub resource_num: i64,

    pub description: String,

    pub auth: bool,

    pub root_path: String,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct ExportCategoryResDto {
    pub category: PortingCategoryObject,
    
    pub subjects: Vec<PortingSubjectObject>,

    pub tags: Vec<PortingTagObject>,

    pub resources: Vec<PortingResourceObject>,
}