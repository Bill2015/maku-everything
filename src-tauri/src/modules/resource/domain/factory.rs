use chrono::Utc;

use crate::modules::category::domain::CategoryID;
use crate::modules::common::domain::ID;
use crate::modules::common::infrastructure::dateutils;

use super::entities::ResourceTaggingEntity;
use super::{Resource, ResourceGenericError, ResourceID, ResourcePlainObject, ResourceProps};
use super::valueobj::{ResourceFileVO, ResourceTaggingVO, ResourceUrlVO};

pub struct ResourceFactory { }

impl ResourceFactory {
    pub fn create(
        name: String,
        description: String,
        belong_category: &CategoryID,
        root_path: String,
        file_path: Option<String>,
        url: Option<String>
    ) -> Result<Resource, ResourceGenericError> {
        let file = match file_path {
            Some(path) if !path.is_empty() => Some(ResourceFileVO::new(&root_path, path)?),
            _ => None,
        };

        let url = match url {
            Some(url) if !url.is_empty() => Some(ResourceUrlVO::new(url)?),
            _ => None,
        };
        
        if name.is_empty() && file.is_none() {
            return Err(ResourceGenericError::NameAndFilePathIsEmpty());
        }
        
        // if no provide resource name, use file name as default
        let new_name = match name.is_empty() {
            true => file.as_ref().unwrap().name.clone(),
            false => name,
        };
        
        Ok(Resource::new(ResourceProps {
            id: ResourceID::new(),
            name: new_name,
            description: description,
            belong_category: belong_category.clone(),
            root_path: root_path,
            file: file,
            url: url,
            auth: false,
            tagging: ResourceTaggingEntity::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }))
    }

    pub fn reconstitute(properties: ResourceProps) -> Resource {
        Resource::new(properties)
    }

    pub fn from_plain(object: ResourcePlainObject) -> Result<Resource, ResourceGenericError> {
        let file = object.file
            .map(|val| ResourceFileVO::new(&object.root_path, val))
            .transpose()?;

        let url = object.url
            .map(|val| ResourceUrlVO::new(val))
            .transpose()?;

        let tags = object.tags
            .into_iter()
            .map(|val| {
                if let Ok(date) = dateutils::parse(&val.added_at) {
                    return Ok(ResourceTaggingVO { id: val.id, added_at: date.and_utc() });
                }
                
                Err(ResourceGenericError::InvalidDateFormat())
            })
            .collect::<Result<Vec<ResourceTaggingVO>, ResourceGenericError>>()?;

        Ok(Resource::new(ResourceProps {
            id: ResourceID::new(),
            name: object.name,
            description: object.description,
            belong_category: object.belong_category,
            root_path: object.root_path,
            file: file,
            url: url,
            auth: object.auth,
            tagging: ResourceTaggingEntity::new(tags),
            created_at: dateutils::parse(&object.created_at)
                .map_err(|_| ResourceGenericError::InvalidDateFormat())?
                .and_utc(),
            updated_at: dateutils::parse(&object.updated_at)
                .map_err(|_| ResourceGenericError::InvalidDateFormat())?
                .and_utc(),
        }))
    }
}
