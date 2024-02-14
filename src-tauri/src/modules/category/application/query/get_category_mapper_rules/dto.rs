use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::modules::common::application::thing_serialize;

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
