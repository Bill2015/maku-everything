use serde::Serialize;
use chrono::{DateTime, Utc};
use crate::common::domain::ID;
use crate::category::domain::CategoryID;

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
    pub fn new(name: String, description: String, belong_category: CategoryID) -> Result<Self, SubjectError> {
        if name.len() <= 0 {
            return  Err(SubjectError::Create(SubjectGenericError::NameIsEmpty()));
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

    pub fn change_name(&mut self, new_name: String) -> Result<(), SubjectError> {
        if new_name.len() <= 0 {
            return  Err(SubjectError::Create(SubjectGenericError::NameIsEmpty()));
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
}