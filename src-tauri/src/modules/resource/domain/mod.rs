use std::fs;
use std::path::Path;

use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::base_aggregate;
use crate::modules::category::domain::CategoryID;
use crate::modules::common::domain::ToPlainObject;
use crate::modules::common::infrastructure::dateutils;
use crate::utils::StringUtils;

mod id;
pub use id::ResourceID;
mod error;
pub use error::ResourceError;
pub use error::ResourceGenericError;
mod plainobj;
pub use plainobj::{ResourcePlainObject, ResourceTaggingPlainObject, ResourceTaggingAttrPlainObject};

pub mod valueobj;
use valueobj::{ResourceFileVO, ResourceUrlVO};

mod factory;
pub use factory::ResourceFactory;

use self::entities::ResourceTaggingEntity;
use self::valueobj::ResourceTaggingAttrVO;
pub mod entities;

// =====================================================
base_aggregate!(Resource {
    id: ResourceID,
    name: String,
    description: String,
    belong_category: CategoryID,
    root_path: String,
    file: Option<ResourceFileVO>,
    url: Option<ResourceUrlVO>,
    auth: bool,
    tagging: ResourceTaggingEntity,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
});

impl Resource {
    pub fn take_id(self) -> ResourceID {
        self.id
    }

    pub fn change_name(&mut self, new_name: String) -> Result<(), ResourceGenericError> {
        if new_name.is_empty() {
            return Err(ResourceGenericError::NameIsEmpty());
        }
        self.name = new_name;
        Ok(())
    }

    pub fn rename_file(&mut self, new_name: Option<String>) -> Result<(), ResourceGenericError> {
        if self.file.is_none() {
            return Err(ResourceGenericError::FileIsEmpty());
        }

        // get new name
        let new_name = new_name.unwrap_or(self.name.clone());
        let ResourceFileVO { uuid, name, path, ext } = self.file.to_owned().unwrap();
    
        // if same as new, do nothing
        if name == new_name {
            return Ok(());
        }

        let file_path = String::from(Path::new(&path).file_name().unwrap().to_str().unwrap());
        let new_path = Path::new(path.slice(..path.chars().count() - file_path.chars().count()))
            .join(&new_name)
            .to_str()
            .unwrap()
            .to_string();
    
        let new_path: String = match &ext {
            Some(ex) => [new_path, ex.to_string()].join("."),
            None => new_path,
        };

        // check filename is already exist (conflict)
        if Path::new(&self.root_path).join(&new_path).exists() {
            return Err(ResourceGenericError::RenameFileFailed());
        }

        fs::rename(
            Path::new(&self.root_path).join(&path),
            Path::new(&self.root_path).join(&new_path)
        ).or(Err(ResourceGenericError::RenameFileFailed()))?;

        self.file = Some(ResourceFileVO {
            name: new_name.clone(),
            path: new_path,
            uuid: uuid,
            ext: ext,
        });

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
        if let Ok(date) = dateutils::parse(new_date) {
            self.updated_at = date.and_utc();
            return Ok(())
        }
        Err(ResourceGenericError::InvalidDateFormat())
    }

    pub fn set_created_at(&mut self, new_date: &str) -> Result<(), ResourceGenericError> {
        if let Ok(date) = dateutils::parse(new_date) {
            self.created_at = date.and_utc();
            return Ok(())
        }
        Err(ResourceGenericError::InvalidDateFormat())
    }

    pub fn set_authorize(&mut self, flag: bool) {
        self.auth = flag;
    }

    pub fn get_mut_tagging(&mut self) -> &mut ResourceTaggingEntity {
        &mut self.tagging
    }
}

impl ToPlainObject<ResourcePlainObject> for Resource {
    fn to_plain(self) -> ResourcePlainObject {
        let tags = self.tagging
            .vals()
            .into_iter()
            .map(move |x| {
                let attrval = match x.attrval.clone() {
                    ResourceTaggingAttrVO::Normal => ResourceTaggingAttrPlainObject::Normal,
                    ResourceTaggingAttrVO::Number(val) => ResourceTaggingAttrPlainObject::Number(val),
                    ResourceTaggingAttrVO::Text(val) => ResourceTaggingAttrPlainObject::Text(val),
                    ResourceTaggingAttrVO::Date(val) => {
                        ResourceTaggingAttrPlainObject::Date(dateutils::format(val))
                    },
                    ResourceTaggingAttrVO::Bool(val) => ResourceTaggingAttrPlainObject::Bool(val),
                };

                ResourceTaggingPlainObject {
                    id: x.id.clone(),
                    added_at: dateutils::format(x.added_at),
                    attrval: attrval,
                }
            })
            .collect::<Vec<ResourceTaggingPlainObject>>();

        ResourcePlainObject {
            id: self.id,
            name: self.name,
            description: self.description,
            belong_category: self.belong_category,
            file: self.file.map(|x| x.path),
            root_path: self.root_path,
            url: self.url.map(|x| x.full),
            created_at: dateutils::format(self.created_at),
            updated_at: dateutils::format(self.updated_at),
            tags: tags,
            auth: self.auth,            
        }
    }
}