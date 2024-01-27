use chrono::NaiveDateTime;
use serde::Serialize;
use chrono::{DateTime, Utc};
use crate::common::domain::Porting;
use crate::common::domain::ID;
use crate::category::domain::CategoryID;
use crate::common::infrastructure::date;

mod id;
pub use id::SubjectID;
mod error;
pub use error::SubjectError;
pub use error::SubjectGenericError;
mod porting;
pub use porting::PortingSubjectObject;

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
    pub fn new(name: String, description: String, belong_category: &CategoryID) -> Result<Self, SubjectGenericError> {
        if name.len() <= 0 {
            return  Err(SubjectGenericError::NameIsEmpty());
        }

        Ok(
            SubjectAggregate {
                id: SubjectID::new(),
                name: name,
                description: description,
                belong_category: belong_category.clone(),
                auth: false,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }
        )
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

impl Porting<PortingSubjectObject> for SubjectAggregate {
    type Err = SubjectGenericError;

    fn import_from(data: PortingSubjectObject) -> Result<Self, Self::Err> {
        let mut new_subject = Self::new(data.name, data.description, &data.belong_category)?;
        new_subject.set_created_at(&data.created_at)?;
        new_subject.set_updated_at(&data.updated_at)?;
        Ok(new_subject)
    }

    fn export_to(self) -> Result<PortingSubjectObject, Self::Err> {
        Ok(PortingSubjectObject {
            id: self.id,
            name: self.name,
            description: self.description,
            belong_category: self.belong_category,
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
            auth: self.auth,
        })
    }
}