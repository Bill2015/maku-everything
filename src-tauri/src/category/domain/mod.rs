use std::path::Path;

use serde::Serialize;
use chrono::{DateTime, Utc};

use crate::common::domain::ID;

mod error;
pub use error::CategoryGenericError;
pub use error::CategoryError;

mod id;
pub use id::CategoryID;


#[derive(Debug, Serialize)]
pub struct CategoryAggregate {
    pub id: CategoryID,
    pub name: String,
    pub description: String,
    pub root_path: String,
    pub auth: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CategoryAggregate {
    pub fn new(name: String, description: String, root_path: String) -> Result<Self, CategoryError> {
        // path can't be empty
        if root_path.is_empty() {
            return Err(CategoryError::Create(CategoryGenericError::RootPathIsEmpty()));
        }

        // the path must be end with '\'
        // TODO: linux path need to impl?
        let new_path = match  root_path.ends_with("\\") {
            true => root_path,
            false => root_path + "\\",
        };

        // create path object
        let path = Path::new(new_path.as_str());
        if path.exists() == false {
            return Err(CategoryError::Create(CategoryGenericError::RootPathNotExists()));
        }

        // name can't be empty
        if name.len() <= 0 {
            return Err(CategoryError::Create(CategoryGenericError::NameIsEmpty()));
        }

        // description can't be empty
        if description.len() <= 0 {
            return  Err(CategoryError::Create(CategoryGenericError::DescriptionIsEmpty()));
        }
        
        Ok(
            CategoryAggregate {
                id: CategoryID::new(),
                name: name,
                description: description,
                root_path: new_path,
                auth: false,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }
        )
    }

    pub fn change_name(&mut self, new_name: String) {
        if new_name.len() <= 0 {
            print!("Name can't be empty");
        }
    
        self.name = new_name;
    }

    pub fn change_description(&mut self, new_description: String) {
        if new_description.len() <= 0 {
            print!("Description can't be empty")
        }
        
        self.description = new_description;
    }

    pub fn change_auth(&mut self, new_auth: bool) {
        self.auth = new_auth;
    }
}
