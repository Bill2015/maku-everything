use serde::{Serialize, Deserialize};
use crate::modules::tag::application::dto::TagAttrDto;

#[derive(Serialize, Deserialize)]
pub struct CreateTagDto {
    pub name: String,

    pub description: String,

    pub belong_category: String,

    pub belong_subject: String,

    #[serde(flatten)]
    pub attrval: TagAttrDto,
}
