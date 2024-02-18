use serde::{Deserialize, Serialize};

use crate::modules::common::infrastructure::dateutils;
use crate::modules::resource::domain::valueobj::{ResourceTaggingAttrVO, ResourceTaggingVO};
use crate::modules::resource::domain::ResourceGenericError;
use crate::modules::tag::domain::valueobj::TagAttrVO;
use crate::modules::tag::domain::{Tag, TagID};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaggingAttrPayload {
    None,

    Number(i64),

    Text(String),

    Date(String),

    Bool(bool),
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ResourceTaggingEntity {
    tags: Vec<ResourceTaggingVO>,

    add_tags: Vec<ResourceTaggingVO>,

    update_tags: Vec<ResourceTaggingVO>,

    del_tags: Vec<TagID>,
}

impl ResourceTaggingEntity {
    pub fn new(tags: Vec<ResourceTaggingVO>) -> Self {
        Self {
            tags,
            add_tags: Vec::new(), 
            del_tags: Vec::new(), 
            update_tags: Vec::new(),
        }
    }

    pub fn add_tag(&mut self, tag: &Tag, payload: Option<TaggingAttrPayload>) -> Result<(), ResourceGenericError> {
        if self.tags.iter().any(|v| v.id == *tag.get_id()) {
            return Err(ResourceGenericError::AddSameTag());
        }

        let attrval = match payload {
            Some(data) => Self::create_tagging_attr(tag, data),
            None => Ok(Self::create_default_tagging_attr(tag))
        }?;

        self.add_tags.push(ResourceTaggingVO::new(tag.get_id().to_string(), attrval));

        Ok(())
    }

    pub fn update_tag(&mut self, tag: &Tag, payload: TaggingAttrPayload) -> Result<(), ResourceGenericError> {
        if self.tags.iter().any(|v| v.id == *tag.get_id()) == false {
            return Err(ResourceGenericError::TagNotExists());
        }

        let attrval = Self::create_tagging_attr(&tag, payload)?;

        self.update_tags.push(ResourceTaggingVO::new(tag.get_id().to_string(), attrval));
        Ok(())
    }

    pub fn del_tag(&mut self, tag_id: &TagID) -> Result<(), ResourceGenericError> {
        if self.tags.iter().any(|v| v.id == *tag_id) == false {
            return Err(ResourceGenericError::TagNotExists());
        }

        self.del_tags.push(tag_id.clone());
        
        Ok(())
    }

    pub fn get_del_tags(&self) -> &Vec<TagID> {
        &self.del_tags
    }

    pub fn get_add_tags(&self) -> &Vec<ResourceTaggingVO> {
        &self.add_tags
    }

    pub fn vals(&self) -> &Vec<ResourceTaggingVO> {
        &self.tags
    }

    pub fn get(self) -> (Vec<ResourceTaggingVO>, Vec<ResourceTaggingVO>, Vec<TagID>) {
        (self.tags, self.add_tags, self.del_tags)
    }

    fn create_default_tagging_attr(tag: &Tag) -> ResourceTaggingAttrVO {
        match tag.get_attr().clone() {
            TagAttrVO::Normal => ResourceTaggingAttrVO::Normal,
            TagAttrVO::Number { defval, .. } => ResourceTaggingAttrVO::Number(defval),
            TagAttrVO::Text { defval } => ResourceTaggingAttrVO::Text(defval),
            TagAttrVO::Date { defval } => ResourceTaggingAttrVO::Date(defval),
            TagAttrVO::Bool { defval } => ResourceTaggingAttrVO::Bool(defval),
        }
    }

    fn create_tagging_attr(tag: &Tag, payload: TaggingAttrPayload) -> Result<ResourceTaggingAttrVO, ResourceGenericError> {
        match tag.get_attr().clone() {
            TagAttrVO::Normal => {
                if let TaggingAttrPayload::None = payload {
                    return Ok(ResourceTaggingAttrVO::Normal);
                }
                Err(ResourceGenericError::InvalidTaggingAttribute())
            }
            TagAttrVO::Number { start, end, .. } => {
                if let TaggingAttrPayload::Number(val) = payload {
                    if val >= start && val <= end {
                        return Ok(ResourceTaggingAttrVO::Number(val));
                    }
                }
                Err(ResourceGenericError::InvalidTaggingAttribute())
            },
            TagAttrVO::Text { .. } => {
                if let TaggingAttrPayload::Text(val) = payload {
                    return Ok(ResourceTaggingAttrVO::Text(val));
                }
                Err(ResourceGenericError::InvalidTaggingAttribute())
            },
            TagAttrVO::Bool { .. } => {
                if let TaggingAttrPayload::Bool(val) = payload {
                    return Ok(ResourceTaggingAttrVO::Bool(val));
                }
                Err(ResourceGenericError::InvalidTaggingAttribute())
            }
            TagAttrVO::Date { .. } => {
                if let TaggingAttrPayload::Date(val) = payload {
                    if let Ok(date) = dateutils::parse(val) {
                        return Ok(ResourceTaggingAttrVO::Date(date.and_utc()));
                    }
                }
                Err(ResourceGenericError::InvalidTaggingAttribute())
            },
        }?
    }
}
