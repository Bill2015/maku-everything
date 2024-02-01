use chrono::NaiveDateTime;
use serde::Serialize;

use crate::modules::common::infrastructure::date;
use crate::modules::resource::domain::valueobj::ResourceTaggingVO;
use crate::modules::resource::domain::{PortingResourceTaggingObject, ResourceGenericError};
use crate::modules::tag::domain::TagID;


#[derive(Debug, Clone, Default, Serialize)]
pub struct ResourceTaggingEntity {
    tags: Vec<ResourceTaggingVO>,

    add_tags: Vec<ResourceTaggingVO>,

    del_tags: Vec<ResourceTaggingVO>,
}

impl ResourceTaggingEntity {
    pub fn new(tags: Vec<ResourceTaggingVO>) -> Self {
        Self { tags, add_tags: Vec::new(), del_tags: Vec::new() }
    }

    pub fn add_tag(&mut self, tag_id: &TagID) -> Result<(), ResourceGenericError> {
        if self.tags.iter().any(|v| v.id == *tag_id) {
            return Err(ResourceGenericError::AddSameTag());
        }

        self.add_tags.push(ResourceTaggingVO::new(tag_id.to_string()));

        Ok(())
    }

    pub fn del_tag(&mut self, tag_id: &TagID) -> Result<(), ResourceGenericError> {
        if self.tags.iter().any(|v| v.id == *tag_id) == false {
            return Err(ResourceGenericError::TagNotExists());
        }

        self.del_tags.push(ResourceTaggingVO::new(tag_id.to_string()));
        
        Ok(())
    }

    pub fn get_del_tags(&self) -> &Vec<ResourceTaggingVO> {
        &self.del_tags
    }

    pub fn get_add_tags(&self) -> &Vec<ResourceTaggingVO> {
        &self.add_tags
    }

    pub fn vals(&self) -> &Vec<ResourceTaggingVO> {
        &self.tags
    }
}
