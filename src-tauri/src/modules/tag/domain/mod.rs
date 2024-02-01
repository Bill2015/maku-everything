
use chrono::{DateTime, Utc};
use crate::modules::common::domain::ID;
use crate::modules::category::domain::CategoryID;
use crate::modules::common::infrastructure::dateutils;
use crate::modules::subject::domain::SubjectID;

mod id;
pub use id::TagID;
mod error;
pub use error::TagError;
pub use error::TagGenericError;
mod porting;
pub use porting::PortingTagObject;

pub struct TagAggregate {
    pub id: TagID,
    pub name: String,
    pub belong_category: CategoryID,
    pub belong_subject: SubjectID,
    pub description: String,
    pub auth: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TagAggregate {
    pub fn new(name: String, description: String, belong_category: &CategoryID, belong_subject: &SubjectID) -> Result<Self, TagGenericError> {
        if name.len() <= 0 {
            return Err(TagGenericError::NameIsEmpty());
        }

        Ok(
            TagAggregate {
                id: TagID::new(),
                name: name,
                belong_category: belong_category.clone(),
                belong_subject: belong_subject.clone(),
                description: description,
                auth: false,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }
        )
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
