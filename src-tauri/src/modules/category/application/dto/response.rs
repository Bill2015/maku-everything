use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::modules::category::domain::CategoryPlainObject;
use crate::modules::subject::domain::SubjectPlainObject;
use crate::modules::resource::domain::ResourcePlainObject;
use crate::modules::common::application::thing_serialize;
use crate::modules::tag::domain::TagPlainObject;

#[derive(Deserialize, Serialize)]
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
    pub category: CategoryPlainObject,
    
    pub subjects: Vec<SubjectPlainObject>,

    pub tags: Vec<TagPlainObject>,

    pub resources: Vec<ResourcePlainObject>,
}


#[derive(Deserialize, Serialize)]
pub struct CategoryMapperRuleItemTagResDto {
    #[serde(serialize_with = "thing_serialize")]
    pub id: Thing,

    pub name: String,

    pub subject_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct CategoryMapperRuleItemResDto {
    pub tag: Option<CategoryMapperRuleItemTagResDto>,

    pub text: String,
}

#[derive(Deserialize, Serialize)]
pub struct CategoryMapperRulesResDto {
    #[serde(serialize_with = "thing_serialize")]
    pub id: Thing,

    pub name: String,

    pub root_path: String,

    pub rules: Vec<CategoryMapperRuleItemResDto>,
}
