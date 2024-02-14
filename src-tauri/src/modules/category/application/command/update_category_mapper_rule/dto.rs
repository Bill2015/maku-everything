use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UpdateCategoryMapperRuleItemDto {
    pub text: String,

    pub tag_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateCategoryMapperRuleDto {
    pub id: String,

    pub rules: Vec<UpdateCategoryMapperRuleItemDto>
}
