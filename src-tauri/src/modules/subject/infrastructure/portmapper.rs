use crate::modules::common::domain::{Porting, ID};
use crate::modules::common::infrastructure::dateutils;
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
            created_at: dateutils::parse(&data.created_at)
                .map_err(|_| SubjectGenericError::InvalidDateFormat())?
                .and_utc(),
            updated_at: dateutils::parse(&data.updated_at)
                .map_err(|_| SubjectGenericError::InvalidDateFormat())?
                .and_utc(),
        };
        Ok(new_subject)
    }

    fn export_to(self) -> Result<PortingSubjectObject, Self::Err> {
        Ok(PortingSubjectObject {
            id: self.id,
            name: self.name,
            description: self.description,
            belong_category: self.belong_category,
            created_at: dateutils::format(self.created_at),
            updated_at: dateutils::format(self.updated_at),
            auth: self.auth,
        })
    }
}
