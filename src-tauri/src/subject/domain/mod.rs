use std::{path::Path};
use serde::Serialize;
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
    pub created_at: String,
    pub updated_at: String,
}

impl SubjectAggregate {
    pub fn new(name: String, description: String, belong_category: CategoryID, auth: bool) -> Self {
        SubjectAggregate {
            id: SubjectID::parse(String::from("")),
            name: name,
            description: description,
            belong_category: belong_category,
            auth: auth,
            created_at: String::from("Create"),
            updated_at: String::from("Update"),
        }
    }

    pub fn change_name(&mut self, new_name: String) {
        if new_name.len() <= 0 {
            print!("Name can't be empty");
        }

        self.name = new_name;
    }
}