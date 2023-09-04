use serde::Serialize;
use chrono::{DateTime, Utc};
use crate::common::domain::ID;

mod id;
pub use id::CategoryID;

#[derive(Debug, Serialize)]
pub struct CategoryAggregate {
    pub id: CategoryID,
    pub name: String,
    pub description: String,
    pub auth: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CategoryAggregate {
    pub fn new(name: String, description: String) -> Self {
        CategoryAggregate {
            id: CategoryID::new(),
            name: name,
            description: description,
            auth: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
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
