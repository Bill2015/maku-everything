use crate::modules::common::domain::{Porting, ID};
use crate::modules::common::infrastructure::dateutils;
use crate::modules::tag::domain::{PortingTagObject, TagAggregate, TagGenericError, TagID};

impl Porting<PortingTagObject> for TagAggregate {
    type Err = TagGenericError;

    fn import_from(data: PortingTagObject) -> Result<Self, Self::Err> {
        let new_tag = Self {
            id: TagID::new(),
            name: data.name,
            belong_category: data.belong_category,
            belong_subject: data.belong_subject,
            description: data.description,
            auth: data.auth,
            created_at: dateutils::parse(&data.created_at)
                .map_err(|_| TagGenericError::InvalidDateFormat())?
                .and_utc(),
            updated_at: dateutils::parse(&data.updated_at)
                .map_err(|_| TagGenericError::InvalidDateFormat())?
                .and_utc(),
        };
        Ok(new_tag)
    }

    fn export_to(self) -> Result<PortingTagObject, Self::Err> {
        Ok(PortingTagObject {
            id: self.id,
            name: self.name,
            description: self.description,
            belong_category: self.belong_category,
            belong_subject: self.belong_subject,
            created_at: dateutils::format(self.created_at),
            updated_at: dateutils::format(self.updated_at),
            auth: self.auth,
        })
    }
}
