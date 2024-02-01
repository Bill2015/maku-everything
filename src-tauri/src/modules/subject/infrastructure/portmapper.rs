use chrono::NaiveDateTime;

use crate::modules::common::domain::{Porting, ID};
use crate::modules::common::infrastructure::date;
use crate::modules::subject::domain::{PortingSubjectObject, SubjectAggregate, SubjectGenericError, SubjectID};

impl Porting<PortingSubjectObject> for SubjectAggregate {
    type Err = SubjectGenericError;

    fn import_from(data: PortingSubjectObject) -> Result<Self, Self::Err> {
        let new_subject = Self {
            id: SubjectID::new(),
            name: data.name,
            description: data.description,
            belong_category: data.belong_category,
            auth: data.auth,
            created_at: NaiveDateTime::parse_from_str(&data.created_at, date::DATE_TIME_FORMAT).unwrap().and_utc(),
            updated_at: NaiveDateTime::parse_from_str(&data.updated_at, date::DATE_TIME_FORMAT).unwrap().and_utc(),
        };
        Ok(new_subject)
    }

    fn export_to(self) -> Result<PortingSubjectObject, Self::Err> {
        Ok(PortingSubjectObject {
            id: self.id,
            name: self.name,
            description: self.description,
            belong_category: self.belong_category,
            created_at: self.created_at.format(date::DATE_TIME_FORMAT).to_string(),
            updated_at: self.updated_at.format(date::DATE_TIME_FORMAT).to_string(),
            auth: self.auth,
        })
    }
}
