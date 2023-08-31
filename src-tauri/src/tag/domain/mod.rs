
use std::{path::Path};
use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::common::domain::ID;
use crate::category::domain::CategoryID;
use crate::subject::domain::SubjectID;

mod id;
pub use id::TagID;

pub struct TagAggregate {
    pub id: TagID,
    pub name: String,
    pub belong_category: CategoryID,
    pub belong_subject: SubjectID,
    pub description: String,
    pub auth: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TagAggregate {
    pub fn new(name: String, description: String, belong_category: CategoryID, belong_subject: SubjectID) -> Self {
        TagAggregate {
            id: TagID::parse(String::from("")),
            name: name,
            belong_category: belong_category,
            belong_subject: belong_subject,
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
            print!("Description can't be empty");
        }

        self.description = new_description;
    }
}
