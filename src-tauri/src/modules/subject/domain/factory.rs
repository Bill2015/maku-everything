use chrono::Utc;

use crate::modules::category::domain::CategoryID;
use crate::modules::common::domain::ID;
use crate::modules::common::infrastructure::dateutils;

use super::{Subject, SubjectGenericError, SubjectID, SubjectPlainObject, SubjectProps};

pub struct SubjectFactory { }

impl SubjectFactory {
    pub fn create(name: String, description: String, belong_category: &CategoryID) -> Result<Subject, SubjectGenericError> {
        if name.len() <= 0 {
            return  Err(SubjectGenericError::NameIsEmpty());
        }

        Ok(Subject::new(SubjectProps {
            id: SubjectID::new(),
            name: name,
            description: description,
            belong_category: belong_category.clone(),
            auth: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }))
    }

    pub fn reconstitute(properties: SubjectProps) -> Subject {
        Subject::new(properties)
    }

    pub fn from_plain(object: SubjectPlainObject) -> Result<Subject, SubjectGenericError> {
        Ok(Subject::new(SubjectProps {
            id: SubjectID::new(),
            name: object.name,
            description: object.description,
            belong_category: object.belong_category,
            auth: object.auth,
            created_at: dateutils::parse(&object.created_at)
                .map_err(|_| SubjectGenericError::InvalidDateFormat())?
                .and_utc(),
            updated_at: dateutils::parse(&object.updated_at)
                .map_err(|_| SubjectGenericError::InvalidDateFormat())?
                .and_utc(),
        }))
    }
}
