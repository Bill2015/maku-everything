use chrono::NaiveDateTime;
use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::modules::category::domain::CategoryID;
use crate::modules::common::infrastructure::date;
use crate::modules::common::domain::ID;

mod id;
pub use id::ResourceID;
mod error;
pub use error::ResourceError;
pub use error::ResourceGenericError;
mod porting;
pub use porting::{PortingResourceObject, PortingResourceTaggingObject};
pub mod valueobj;
use valueobj::{ResourceFileVO, ResourceUrlVO};

use self::entities::ResourceTaggingEntity;
pub mod entities;


// =====================================================
#[derive(Debug, Serialize)]
pub struct ResourceAggregate {
    pub id: ResourceID,
    pub name: String,
    pub description: String,
    pub belong_category: CategoryID,
    pub root_path: String,
    pub file: Option<ResourceFileVO>,
    pub url: Option<ResourceUrlVO>,
    pub auth: bool,
    pub tagging: ResourceTaggingEntity,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ResourceAggregate {

    pub fn new(
        name: String,
        description: String,
        belong_category: &CategoryID,
        root_path: String,
        file_path: Option<String>,
        url: Option<String>
    ) -> Result<Self, ResourceGenericError> {
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
        
        Ok(ResourceAggregate {
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
        })
    }

    pub fn change_name(&mut self, new_name: String) -> Result<(), ResourceGenericError> {
        if new_name.is_empty() {
            return Err(ResourceGenericError::NameIsEmpty());
        }
        self.name = new_name;
        Ok(())
    }

    pub fn change_description(&mut self, new_description: String) -> Result<(), ResourceGenericError> {
        self.description = new_description;
        Ok(())
    }

    pub fn change_file(&mut self, file_path: String) -> Result<(), ResourceGenericError> {
        self.file = Some(ResourceFileVO::new(&self.root_path, file_path)?);
        Ok(())
    }

    pub fn set_updated_at(&mut self, new_date: &str) -> Result<(), ResourceGenericError>{
        if let Ok(date) = NaiveDateTime::parse_from_str(new_date, date::DATE_TIME_FORMAT) {
            self.updated_at = date.and_utc();
            return Ok(())
        }
        Err(ResourceGenericError::InvalidDateFormat())
    }

    pub fn set_created_at(&mut self, new_date: &str) -> Result<(), ResourceGenericError> {
        if let Ok(date) = NaiveDateTime::parse_from_str(new_date, date::DATE_TIME_FORMAT) {
            self.created_at = date.and_utc();
            return Ok(())
        }
        Err(ResourceGenericError::InvalidDateFormat())
    }

    pub fn set_authorize(&mut self, flag: bool) {
        self.auth = flag;
    }

    pub fn get_tagging(&self) -> &ResourceTaggingEntity {
        &self.tagging
    }

    pub fn get_mut_tagging(&mut self) -> &mut ResourceTaggingEntity {
        &mut self.tagging
    }
}
