use chrono::NaiveDateTime;

use crate::modules::common::domain::{Porting, ID};
use crate::modules::common::infrastructure::date;
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
            created_at: NaiveDateTime::parse_from_str(&data.created_at, date::DATE_TIME_FORMAT).unwrap().and_utc(),
            updated_at: NaiveDateTime::parse_from_str(&data.updated_at, date::DATE_TIME_FORMAT).unwrap().and_utc(),
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
            created_at: self.created_at.format(date::DATE_TIME_FORMAT).to_string(),
            updated_at: self.updated_at.format(date::DATE_TIME_FORMAT).to_string(),
            auth: self.auth,
        })
    }
}
