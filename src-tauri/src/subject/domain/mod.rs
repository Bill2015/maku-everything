use serde::Serialize;
use chrono::{DateTime, Utc};
use crate::common::domain::ID;
use crate::category::domain::CategoryID;

mod id;
pub use id::SubjectID;

#[derive(Debug, Serialize)]
pub struct SubjectAggregate {
    pub id: SubjectID,
    pub name: String,
    pub description: String,
    pub belong_category: CategoryID,
    pub auth: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SubjectAggregate {
    pub fn new(name: String, description: String, belong_category: CategoryID) -> Self {
        SubjectAggregate {
            id: SubjectID::parse(String::from("")),
            name: name,
            description: description,
            belong_category: belong_category,
            auth: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn change_name(&mut self, new_name: String) {
        if new_name.len() <= 0 {
            println!("Name can't be empty");
        }

        self.name = new_name;
    }

    pub fn change_description(&mut self, new_description: String) {
        if new_description.len() <= 0 {
            println!("Description can't be empty")
        }

        self.description = new_description;
    }
}