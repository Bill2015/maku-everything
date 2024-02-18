use chrono::Utc;

use crate::modules::category::domain::CategoryID;
use crate::modules::subject::domain::SubjectID;
use crate::modules::common::domain::ID;
use crate::modules::common::infrastructure::dateutils;

use super::plainobj::TagAttributePlainObject;
use super::{Tag, TagGenericError, TagID, TagPlainObject, TagProps};
use super::valueobj::TagAttrVO;

pub struct TagFactory { }

impl TagFactory {
    pub fn create(name: String, description: String, belong_category: &CategoryID, belong_subject: &SubjectID, attr: TagAttrVO) -> Result<Tag, TagGenericError> {
        if name.len() <= 0 {
            return  Err(TagGenericError::NameIsEmpty());
        }

        Ok(Tag::new(TagProps {
            id: TagID::new(),
            name: name,
            belong_category: belong_category.clone(),
            belong_subject: belong_subject.clone(),
            description: description,
            auth: false,
            attr: attr,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }))
    }

    pub fn reconstitute(properties: TagProps) -> Tag {
        Tag::new(properties)
    }

    pub fn from_plain(object: TagPlainObject) -> Result<Tag, TagGenericError> {
        let attr = match object.attrval {
            TagAttributePlainObject::Normal => TagAttrVO::Normal,
            TagAttributePlainObject::Number { start, end, defval } => TagAttrVO::Number { start, end, defval },
            TagAttributePlainObject::Text { defval } => TagAttrVO::Text { defval },
            TagAttributePlainObject::Date { defval } => {
                TagAttrVO::Date {
                    defval: dateutils::parse(defval)
                        .map_err(|_| TagGenericError::InvalidDateFormat())?
                        .and_utc()
                }
            },
            TagAttributePlainObject::Bool { defval } => TagAttrVO::Bool { defval },
        };

        Ok(Tag::new(TagProps {
            id: TagID::new(),
            name: object.name,
            belong_category: object.belong_category,
            belong_subject: object.belong_subject,
            description: object.description,
            auth: object.auth,
            attr: attr,
            created_at: dateutils::parse(&object.created_at)
                .map_err(|_| TagGenericError::InvalidDateFormat())?
                .and_utc(),
            updated_at: dateutils::parse(&object.updated_at)
                .map_err(|_| TagGenericError::InvalidDateFormat())?
                .and_utc(),
        }))
    }
}
