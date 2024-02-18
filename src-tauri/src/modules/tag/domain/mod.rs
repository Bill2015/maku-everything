use serde::Serialize;
use chrono::{DateTime, Utc};

use crate::base_aggregate;
use crate::modules::category::domain::CategoryID;
use crate::modules::common::domain::ToPlainObject;
use crate::modules::common::infrastructure::dateutils;
use crate::modules::subject::domain::SubjectID;

mod id;
pub use id::TagID;
mod error;
pub use error::TagError;
pub use error::TagGenericError;
mod factory;
pub use factory::TagFactory;
pub mod valueobj;
mod plainobj;
pub use plainobj::TagPlainObject;

use valueobj::TagAttrVO;

use self::plainobj::TagAttributePlainObject;

base_aggregate!(Tag {
    id: TagID,
    name: String,
    belong_category: CategoryID,
    belong_subject: SubjectID,
    description: String,
    auth: bool,
    attr: TagAttrVO,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
});

impl Tag {
    pub fn take_id(self) -> TagID {
        self.id
    }

    pub fn change_name(&mut self, new_name: String) -> Result<(), TagGenericError> {
        if new_name.is_empty() {
            return Err(TagGenericError::NameIsEmpty());
        }
        self.name = new_name;
        Ok(())
    }

    pub fn change_description(&mut self, new_description: String) -> Result<(), TagGenericError> {
        if new_description.is_empty() {
            return Err(TagGenericError::DescriptionIsEmpty());
        }
        self.description = new_description;
        Ok(())
    }

    pub fn set_updated_at(&mut self, new_date: &str) -> Result<(), TagGenericError> {
        if let Ok(date) = dateutils::parse(new_date) {
            self.updated_at = date.and_utc();
            return Ok(())
        }
        Err(TagGenericError::InvalidDateFormat())
    }

    pub fn set_created_at(&mut self, new_date: &str) -> Result<(), TagGenericError>  {
        if let Ok(date) = dateutils::parse(new_date) {
            self.created_at = date.and_utc();
            return Ok(())
        }
        Err(TagGenericError::InvalidDateFormat())
    }
}

impl ToPlainObject<TagPlainObject> for Tag {
    fn to_plain(self) -> TagPlainObject {
        let attrobj = match self.attr {
            TagAttrVO::Normal => TagAttributePlainObject::Normal,
            TagAttrVO::WithNumber { start, end, defval } => TagAttributePlainObject::Number { start, end, defval },
            TagAttrVO::WithText { defval } => TagAttributePlainObject::Text { defval },
            TagAttrVO::WithDate { defval } => TagAttributePlainObject::Date { defval: dateutils::format(defval) },
            TagAttrVO::WithBool { defval } => TagAttributePlainObject::Bool { defval },
        };

        TagPlainObject {
            id: self.id,
            name: self.name,
            description: self.description,
            belong_category: self.belong_category,
            belong_subject: self.belong_subject,
            created_at: dateutils::format(self.created_at),
            updated_at: dateutils::format(self.updated_at),
            attrval: attrobj,
            auth: self.auth,
        }
    }
}
