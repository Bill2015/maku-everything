use chrono::NaiveDateTime;
use serde::Serialize;
use chrono::{DateTime, Utc};
use crate::common::domain::ID;
use crate::category::domain::CategoryID;
use crate::common::infrastructure::date;

mod id;
pub use id::SubjectID;
mod error;
pub use error::SubjectError;
pub use error::SubjectGenericError;

#[derive(Debug, Serialize)]
pub struct SubjectAggregate {
    pub id: SubjectID,
    pub name: String,
    pub description: String,
    pub belong_category: CategoryID,
    pub auth: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SubjectAggregate {
    pub fn new(name: String, description: String, belong_category: CategoryID) -> Result<Self, SubjectGenericError> {
        if name.len() <= 0 {
            return  Err(SubjectGenericError::NameIsEmpty());
        }

        Ok(
            SubjectAggregate {
                id: SubjectID::new(),
                name: name,
                description: description,
                belong_category: belong_category,
                auth: false,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }
        )
    }

    pub fn change_name(&mut self, new_name: String) -> Result<(), SubjectGenericError> {
        if new_name.len() <= 0 {
            return  Err(SubjectGenericError::NameIsEmpty());
        }

        self.name = new_name;
        Ok(())
    }

    pub fn change_description(&mut self, new_description: String) {
        if new_description.len() <= 0 {
            println!("Description can't be empty")
        }

        self.description = new_description;
    }

    pub fn set_updated_at(&mut self, new_date: &str) -> Result<(), SubjectGenericError> {
        if let Ok(date) = NaiveDateTime::parse_from_str(new_date, date::DATE_TIME_FORMAT) {
            self.updated_at = date.and_utc();
            return Ok(())
        }
        Err(SubjectGenericError::InvalidDateFormat())
    }

    pub fn set_created_at(&mut self, new_date: &str) -> Result<(), SubjectGenericError> {
        if let Ok(date) = NaiveDateTime::parse_from_str(new_date, date::DATE_TIME_FORMAT) {
            self.created_at = date.and_utc();
            return Ok(())
        }
        Err(SubjectGenericError::InvalidDateFormat())
    }
}