
use std::{path::Path};
use serde::Serialize;
use crate::common::domain::ID;

mod id;
pub use id::TagID;

pub struct TagAggregate {
    pub id: TagID,
    pub name: String,
    pub belong_category: String,
    pub description: String,
    pub auth: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl TagAggregate {
    pub fn new(name: String, belong_category: String, description: String, auth: bool) -> Self {
        TagAggregate {
            id: TagID::parse(String::from("")),
            name: name,
            belong_category: belong_category,
            description: description,
            auth: auth,
            created_at: String::from("Create"),
            updated_at: String::from("Update"),
        }
    }
}