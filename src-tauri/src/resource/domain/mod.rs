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
    pub fn new(root_path: String, file_path: String) -> Result<Option<Self>, String> {
        
        if file_path.is_empty() {
            return Ok(None);
        }

        // If path already contain root path
        // trim it and re-concat it
        let main_path = file_path.trim_start_matches(&root_path);
        let full_path = root_path + main_path;

        // concat with root path
        let path = Path::new(full_path.as_str());

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
                    path: String::from(main_path),
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
    pub name: String,
    pub description: String,
    pub belong_category: CategoryID,
    pub file: Option<ResourceFileAggregate>,
    pub auth: bool,
    pub tags: Vec<TagID>,
    pub new_tags: Vec<TagID>,
    pub del_tags: Vec<TagID>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ResourceAggregate {

    pub fn new(name: String, description: String, belong_category: CategoryID, root_path: String, file_path: String) -> Result<Self, String> {
        let file = ResourceFileAggregate::new(root_path, file_path)?;
        
        if name.is_empty() && file.is_none() {
            return Err(String::from("Create Resource Error"));
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
            belong_category: belong_category,
            file: file,
            auth: false,
            tags: Vec::new(),
            new_tags: Vec::new(),
            del_tags: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn change_name(&mut self, new_name: String) {
        if new_name.len() <= 0 {
            println!("Name can't be empty");
        }

        self.name = new_name;
    }

    pub fn change_description(&mut self, new_description: String) {
        if new_description.len() <= 0 {
            print!("Description can't be empty");
        }

        self.description = new_description;
    }

    pub fn change_file(&mut self, root_path: String, file_path: String) -> Result<(), String> {
        self.file = ResourceFileAggregate::new(root_path, file_path)?;
        Ok(())
    }

    pub fn set_authorize(&mut self, flag: bool) {
        self.auth = flag;
    }

    pub fn add_tag(&mut self, tag_id: TagID) -> Result<(), String> {
        if self.tags.contains(&tag_id) {
            return Err(String::from("Cannot Add same tags"));
        }

        self.new_tags.push(tag_id);

        Ok(())
    }

    pub fn del_tag(&mut self, tag_id: TagID) -> Result<(), String> {
        dbg!(&tag_id);
        dbg!(&self.tags);
        if self.tags.contains(&tag_id) == false {
            return Err(String::from("Cannot remove not existed tags"));
        }
    
        self.del_tags.push(tag_id);
        
        Ok(())
    }

}
