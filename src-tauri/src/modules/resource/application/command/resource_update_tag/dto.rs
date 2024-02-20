use serde::{Serialize, Deserialize};

use crate::modules::resource::application::dto::ResourceTaggingAttrPayloadDto;

#[derive(Serialize, Deserialize)]
pub struct ResourceUpdateTagDto {
    pub id: String,

    pub tag_id: String,

    pub attrval: ResourceTaggingAttrPayloadDto,
}
