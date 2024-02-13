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
mod plainobj;
pub use plainobj::TagPlainObject;

base_aggregate!(Tag {
    id: TagID,
    name: String,
    belong_category: CategoryID,
    belong_subject: SubjectID,
    description: String,
    auth: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
});

impl Tag {
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
        TagPlainObject {
            id: self.id,
            name: self.name,
            description: self.description,
            belong_category: self.belong_category,
            belong_subject: self.belong_subject,
            created_at: dateutils::format(self.created_at),
            updated_at: dateutils::format(self.updated_at),
            auth: self.auth,
        }
    }
}
