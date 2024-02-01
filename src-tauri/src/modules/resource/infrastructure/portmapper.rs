use chrono::NaiveDateTime;

use crate::modules::common::domain::{Porting, ID};
use crate::modules::common::infrastructure::date;
use crate::modules::resource::domain::entities::ResourceTaggingEntity;
use crate::modules::resource::domain::valueobj::{ResourceFileVO, ResourceTaggingVO, ResourceUrlVO};
use crate::modules::resource::domain::{PortingResourceObject, PortingResourceTaggingObject, ResourceAggregate, ResourceGenericError, ResourceID};

impl Porting<Vec<PortingResourceTaggingObject>> for ResourceTaggingEntity {
    type Err = ResourceGenericError;

    fn import_from(data: Vec<PortingResourceTaggingObject>) -> Result<Self, Self::Err> {
        let tags = data
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

    fn export_to(self) -> Result<Vec<PortingResourceTaggingObject>, Self::Err> {
        Ok(self.vals()
            .into_iter()
            .map(move |x| PortingResourceTaggingObject {
                id: x.id.clone(),
                added_at: x.added_at.format(date::DATE_TIME_FORMAT).to_string(),
            })
            .collect::<Vec<PortingResourceTaggingObject>>()
        )
    }
}

impl Porting<PortingResourceObject> for ResourceAggregate {
    type Err = ResourceGenericError;

    fn import_from(data: PortingResourceObject) -> Result<Self, Self::Err> {
        let file = data.file
            .map(|val| ResourceFileVO::new(&data.root_path, val))
            .transpose()?;

        let url = data.url
            .map(|val| ResourceUrlVO::new(val))
            .transpose()?;

        let tagging = ResourceTaggingEntity::import_from(data.tags)?;

        let new_res = ResourceAggregate {
            id: ResourceID::new(),
            name: data.name,
            description: data.description,
            belong_category: data.belong_category,
            root_path: data.root_path,
            file: file,
            url: url,
            auth: data.auth,
            tagging: tagging,
            created_at: NaiveDateTime::parse_from_str(&data.created_at, date::DATE_TIME_FORMAT).unwrap().and_utc(),
            updated_at: NaiveDateTime::parse_from_str(&data.updated_at, date::DATE_TIME_FORMAT).unwrap().and_utc(),
        };

        Ok(new_res)
    }

    fn export_to(self) -> Result<PortingResourceObject, Self::Err> {
        Ok(PortingResourceObject {
            id: self.id,
            name: self.name,
            description: self.description,
            belong_category: self.belong_category,
            file: self.file.map(|x| x.path),
            root_path: self.root_path,
            url: self.url.map(|x| x.full),
            created_at: self.created_at.format(date::DATE_TIME_FORMAT).to_string(),
            updated_at: self.updated_at.format(date::DATE_TIME_FORMAT).to_string(),
            tags: self.tagging.export_to()?,
            auth: self.auth,            
        })
    }
}
