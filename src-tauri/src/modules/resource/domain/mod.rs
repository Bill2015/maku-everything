use chrono::NaiveDateTime;
use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::modules::category::domain::CategoryID;
use crate::modules::common::infrastructure::date;
use crate::modules::tag::domain::TagID;
use crate::modules::common::domain::{Porting, ID};


mod id;
pub use id::ResourceID;
mod error;
pub use error::ResourceError;
pub use error::ResourceGenericError;
mod porting;
pub use porting::PortingResourceObject;
pub mod valueobj;
use valueobj::{ResourceFileVO, ResourceUrlVO};

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
    pub tags: Vec<TagID>,
    pub new_tags: Vec<TagID>,
    pub del_tags: Vec<TagID>,
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
            tags: Vec::new(),
            new_tags: Vec::new(),
            del_tags: Vec::new(),
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

    pub fn add_tag(&mut self, tag_id: &TagID) -> Result<(), ResourceGenericError> {
        if self.tags.contains(tag_id) {
            return Err(ResourceGenericError::AddSameTag());
        }

        self.new_tags.push(tag_id.clone());

        Ok(())
    }

    pub fn del_tag(&mut self, tag_id: &TagID) -> Result<(), ResourceGenericError> {
        if self.tags.contains(tag_id) == false {
            return Err(ResourceGenericError::TagNotExists());
        }
    
        self.del_tags.push(tag_id.clone());
        
        Ok(())
    }

}

impl Porting<PortingResourceObject> for ResourceAggregate {
    type Err = ResourceGenericError;

    fn import_from(data: PortingResourceObject) -> Result<Self, Self::Err> {
        let mut new_res = ResourceAggregate::new(
            data.name,
            data.description,
            &data.belong_category,
            data.root_path,
            None,
            data.url,
        )?;
        new_res.set_created_at(&data.created_at)?;
        new_res.set_updated_at(&data.updated_at)?;

        if let Some(file) = data.file {
            new_res.change_file(file)?;
        }

        for tag in data.tags {
            new_res.add_tag(&tag)?;
        }

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
            tags: self.tags,
            auth: self.auth,            
        })
    }

    
}