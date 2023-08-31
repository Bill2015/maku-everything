use std::ffi::OsStr;
use std::{path::Path};
use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::category::domain::CategoryID;
use crate::tag::domain::TagID;
use crate::common::domain::ID;


mod id;
pub use id::ResourceID;

#[derive(Debug, Serialize)]
pub struct ResourceFileAggregate {
    pub uuid: String,
    pub name: String,
    pub path: String,
    pub ext: String,
}

impl ResourceFileAggregate {
    pub fn new(file_path: String) -> Result<Option<Self>, String> {
        
        if file_path.is_empty() {
            return Ok(None);
        }

        let path = Path::new(file_path.as_str());

        if path.exists() == false {
            return Err(String::from("Path not exist"));
        }
        
        if path.file_name().is_none() {
            return Err(String::from("No file name"));
        }

        let ext = match path.is_file() {
            true => path.extension().unwrap_or(OsStr::new("txt")),
            false => OsStr::new("folder"),
        };

        Ok(
            Some(
                ResourceFileAggregate {
                    uuid: String::from("id"),
                    name: String::from(path.file_name().unwrap().to_str().unwrap()),
                    ext: String::from(ext.to_str().unwrap()),
                    path: file_path,
                }
            )
        )
    }

    pub fn from_do(uuid: String, name: String, path: String, ext: String) -> Self {
        ResourceFileAggregate {
            uuid: uuid,
            name: name,
            path: path,
            ext: ext,
        }
    } 
}

// =====================================================
#[derive(Debug, Serialize)]
pub struct ResourceAggregate {
    pub id: ResourceID,
    pub title: String,
    pub description: String,
    pub belong_category: CategoryID,
    pub file: Option<ResourceFileAggregate>,
    pub auth: bool,
    pub tags: Vec<TagID>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ResourceAggregate {

    pub fn new(title: String, description: String, belong_category: CategoryID, file_path: String) -> Result<Self, String> {
        Ok(ResourceAggregate {
            id: ResourceID::new(),
            title: title,
            description: description,
            belong_category: belong_category,
            file: ResourceFileAggregate::new(file_path)?,
            auth: false,
            tags: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn change_title(&mut self, new_title: String) {
        if new_title.len() <= 0 {
            println!("Title can't be empty");
        }

        self.title = new_title;
    }

    pub fn change_description(&mut self, new_description: String) {
        if new_description.len() <= 0 {
            print!("Description can't be empty");
        }

        self.description = new_description;
    }

    pub fn change_file(&mut self, file_path: String) -> Result<(), String> {
        self.file = ResourceFileAggregate::new(file_path)?;
        Ok(())
    }

    pub fn set_authorize(&mut self, flag: bool) {
        self.auth = flag;
    }

    pub fn add_tag(&mut self, tag_id: TagID) {
        if self.tags.contains(&tag_id) {
            println!("Cannot Add same tags");
        }
        else {
            self.tags.push(tag_id);
        }
    }

    pub fn remove_tag(&mut self, tag_id: TagID) {
        if let Some(index) = self.tags.iter().position(|x| x == &tag_id) {
            self.tags.remove(index);
        }
        else {
            println!("Cannot remove not existed tags");
        }
    }

}
