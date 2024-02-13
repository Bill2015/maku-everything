use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::base_aggregate;
use crate::modules::category::domain::CategoryID;
use crate::modules::common::domain::ToPlainObject;
use crate::modules::common::infrastructure::dateutils;

mod id;
pub use id::ResourceID;
mod error;
pub use error::ResourceError;
pub use error::ResourceGenericError;
mod plainobj;
pub use plainobj::{ResourcePlainObject, ResourceTaggingPlainObject};

pub mod valueobj;
use valueobj::{ResourceFileVO, ResourceUrlVO};

mod factory;
pub use factory::ResourceFactory;

use self::entities::ResourceTaggingEntity;
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
            .map(move |x| ResourceTaggingPlainObject {
                id: x.id.clone(),
                added_at: dateutils::format(x.added_at),
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