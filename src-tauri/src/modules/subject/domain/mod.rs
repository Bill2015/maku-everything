use serde::Serialize;
use chrono::{DateTime, Utc};

use crate::base_aggregate;
use crate::modules::common::domain::ToPlainObject;
use crate::modules::category::domain::CategoryID;
use crate::modules::common::infrastructure::dateutils;

mod id;
pub use id::SubjectID;
mod error;
pub use error::SubjectError;
pub use error::SubjectGenericError;
mod factory;
pub use factory::SubjectFactory;
mod plainobj;
pub use plainobj::SubjectPlainObject;

base_aggregate!(Subject {
    id: SubjectID,
    name: String,
    description: String,
    belong_category: CategoryID,
    auth: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
});

impl Subject {
    pub fn take_id(self) -> SubjectID {
        self.id
    }

    pub fn change_name(&mut self, new_name: String) -> Result<(), SubjectGenericError> {
        if new_name.is_empty() {
            return Err(SubjectGenericError::NameIsEmpty());
        }

        self.name = new_name;
        Ok(())
    }

    pub fn change_description(&mut self, new_description: String) -> Result<(), SubjectGenericError> {
        if new_description.is_empty() {
            return Err(SubjectGenericError::DescriptionIsEmpty());
        }
        self.description = new_description;
        Ok(())
    }

    pub fn set_updated_at(&mut self, new_date: &str) -> Result<(), SubjectGenericError> {
        if let Ok(date) = dateutils::parse(new_date) {
            self.updated_at = date.and_utc();
            return Ok(())
        }
        Err(SubjectGenericError::InvalidDateFormat())
    }

    pub fn set_created_at(&mut self, new_date: &str) -> Result<(), SubjectGenericError> {
        if let Ok(date) = dateutils::parse(new_date) {
            self.created_at = date.and_utc();
            return Ok(())
        }
        Err(SubjectGenericError::InvalidDateFormat())
    }
}

impl ToPlainObject<SubjectPlainObject> for Subject {
    fn to_plain(self) -> SubjectPlainObject {
        SubjectPlainObject {
            id: self.id,
            name: self.name,
            description: self.description,
            belong_category: self.belong_category,
            created_at: dateutils::format(self.created_at),
            updated_at: dateutils::format(self.updated_at),
            auth: self.auth,
        }
    }
}
