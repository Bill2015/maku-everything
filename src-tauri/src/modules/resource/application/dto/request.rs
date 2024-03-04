use serde::{Serialize, Deserialize};

use crate::modules::resource::domain::entities::TaggingAttrPayload;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResourceTaggingAttrPayloadDto {
    None,
    Number(i64),
    Text(String),
    Bool(bool),
}
impl Into<TaggingAttrPayload> for ResourceTaggingAttrPayloadDto {
    fn into(self) -> TaggingAttrPayload {
        match self {
            Self::None => TaggingAttrPayload::None,
            Self::Number(val) => TaggingAttrPayload::Number(val),
            Self::Text(val) => TaggingAttrPayload::Text(val),
            Self::Bool(val) => TaggingAttrPayload::Bool(val),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ResourceIdOnlyDto {
    id: String,
}
