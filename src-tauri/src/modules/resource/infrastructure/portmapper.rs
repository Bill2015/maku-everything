use chrono::NaiveDateTime;

use crate::modules::common::infrastructure::date;
use crate::modules::resource::domain::entities::ResourceTaggingEntity;
use crate::modules::resource::domain::valueobj::ResourceTaggingVO;
use crate::modules::resource::domain::{PortingResourceTaggingObject, ResourceGenericError};

impl TryFrom<Vec<PortingResourceTaggingObject>> for ResourceTaggingEntity {
    type Error = ResourceGenericError;

    fn try_from(value: Vec<PortingResourceTaggingObject>) -> Result<Self, Self::Error> {
        let tags = value
            .into_iter()
            .map(|val| {
                if let Ok(date) = NaiveDateTime::parse_from_str(&val.added_at, date::DATE_TIME_FORMAT) {
                    return Ok(ResourceTaggingVO { id: val.id, added_at: date.and_utc() });
                }
                
                Err(ResourceGenericError::InvalidDateFormat())
            })
            .collect::<Result<Vec<ResourceTaggingVO>, ResourceGenericError>>()?;

        Ok(Self::new(tags))
    }
}

impl Into<Vec<PortingResourceTaggingObject>> for ResourceTaggingEntity {
    fn into(self) -> Vec<PortingResourceTaggingObject> {
        self.vals()
            .into_iter()
            .map(move |x| PortingResourceTaggingObject {
                id: x.id.clone(),
                added_at: x.added_at.format(date::DATE_TIME_FORMAT).to_string(),
            })
            .collect::<Vec<PortingResourceTaggingObject>>()
    }
}
