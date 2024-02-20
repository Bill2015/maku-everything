use serde::{Serialize, Deserialize};

use crate::modules::resource::application::dto::ResourceTaggingAttrPayloadDto;

#[derive(Serialize, Deserialize)]
pub struct ResourceAddTagDto {
    pub id: String,

    pub tag_id: String,

    pub attrval: Option<ResourceTaggingAttrPayloadDto>,
}

