
use chrono::NaiveDateTime;
use chrono::{DateTime, Utc};
use crate::common::domain::ID;
use crate::category::domain::CategoryID;
use crate::common::infrastructure::date;
use crate::subject::domain::SubjectID;

mod id;
pub use id::TagID;
mod error;
pub use error::TagError;
pub use error::TagGenericError;

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
    pub fn new(name: String, description: String, belong_category: CategoryID, belong_subject: SubjectID) -> Result<Self, TagError> {
        if name.len() <= 0 {
            return Err(TagError::Create(TagGenericError::NameIsEmpty()));
        }

        Ok(
            TagAggregate {
                id: TagID::new(),
                name: name,
                belong_category: belong_category,
                belong_subject: belong_subject,
                description: description,
                auth: false,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }
        )
    }

    pub fn change_name(&mut self, new_name: String) -> Result<(), TagError>{
        if new_name.len() <= 0 {
            return Err(TagError::Update(TagGenericError::NameIsEmpty()));
        }

        self.name = new_name;
        Ok(())
    }

    pub fn change_description(&mut self, new_description: String) {
        if new_description.len() <= 0 {
            print!("Description can't be empty");
        }

        self.description = new_description;
    }

    pub fn set_updated_at(&mut self, new_date: &str) {
        if let Ok(date) = NaiveDateTime::parse_from_str(new_date, date::DATE_TIME_FORMAT) {
            self.updated_at = date.and_utc();
        }
    }

    pub fn set_created_at(&mut self, new_date: &str) {
        if let Ok(date) = NaiveDateTime::parse_from_str(new_date, date::DATE_TIME_FORMAT) {
            self.created_at = date.and_utc();
        }
    }
}
