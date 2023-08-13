use std::{path::Path};
use serde::Serialize;
use crate::category::domain::CategoryID;
use crate::tag::domain::TagID;
use crate::common::domain::ID;


mod id;
pub use id::ResourceID;

// =====================================================
#[derive(Debug, Serialize)]
pub struct ResourceAggregate {
    pub id: ResourceID,
    pub title: String,
    pub description: String,
    pub belong_category: CategoryID,
    pub file_id: String,
    pub file_name: String,
    pub file_path: String,
    pub file_type: String,
    pub auth: bool,
    pub tags: Vec<TagID>,
    pub created_at: String,
    pub updated_at: String,
}

impl ResourceAggregate {

    pub fn new(title: String, description: String, belong_category: CategoryID, file_id: String, file_name: String, file_path: String, file_type: String, auth: bool) -> Self {
        ResourceAggregate {
            id: ResourceID::parse(String::from("")),
            title: title,
            description: description,
            belong_category: belong_category,
            file_id: file_id,
            file_name: file_name,
            file_path: file_path,
            file_type: file_type,
            auth: auth,
            tags: Vec::new(),
            created_at: String::from("Create"),
            updated_at: String::from("Update"),
        }
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

    pub fn change_file(&mut self, file_path: String) {
        if file_path.len() <= 0 {
            println!("File Path can't be empty");
        }

        let path = Path::new(&file_path);

        self.file_path = file_path.clone();
        self.file_name = String::from(path.file_name().unwrap_or_default().to_str().unwrap());
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
