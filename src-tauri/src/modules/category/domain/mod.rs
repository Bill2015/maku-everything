use std::path::Path;

use serde::Serialize;
use chrono::{DateTime, Utc};

use crate::modules::common::domain::ID;
use crate::modules::common::infrastructure::dateutils;

mod entities;
pub use entities::RuleTableEntity;

mod valueobj;
pub use valueobj::RuleItemVO;

mod error;
pub use error::CategoryGenericError;
pub use error::CategoryError;

mod id;
pub use id::CategoryID;

mod porting;
pub use porting::PortingCategoryObject;

#[derive(Debug, Serialize)]
pub struct CategoryAggregate {
    pub id: CategoryID,
    pub name: String,
    pub description: String,
    pub root_path: String,
    pub auth: bool,
    pub rule_table: RuleTableEntity,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CategoryAggregate {
    pub fn relove_path(path: String) -> Result<String, CategoryGenericError> {
        // path can't be empty
        if path.is_empty() {
            return Err(CategoryGenericError::RootPathIsEmpty());
        }

        // the path must be end with '\'
        // TODO: linux path need to impl?
        let new_path = match path.ends_with("\\") {
            true => path,
            false => path + "\\",
        };

        // create path object
        let path = Path::new(&new_path);
        if path.exists() == false {
            return Err(CategoryGenericError::RootPathNotExists());
        }

        Ok(new_path)
    }

    pub fn new(name: String, description: String, root_path: String) -> Result<Self, CategoryGenericError> {

        let new_path = Self::relove_path(root_path)?;

        // name can't be empty
        if name.len() <= 0 {
            return Err(CategoryGenericError::NameIsEmpty());
        }

        Ok(
            CategoryAggregate {
                id: CategoryID::new(),
                name: name,
                description: description,
                root_path: new_path,
                auth: false,
                rule_table: RuleTableEntity::new(Vec::new()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }
        )
    }

    pub fn set_updated_at(&mut self, new_date: &str) -> Result<(), CategoryGenericError> {
        if let Ok(date) = dateutils::parse(new_date) {
            self.updated_at = date.and_utc();
            return Ok(())
        }
        Err(CategoryGenericError::InvalidDateFormat())
    }

    pub fn set_created_at(&mut self, new_date: &str) -> Result<(), CategoryGenericError> {
        if let Ok(date) = dateutils::parse(new_date) {
            self.created_at = date.and_utc();
            return Ok(())
        }
        Err(CategoryGenericError::InvalidDateFormat())
    }

    pub fn change_name(&mut self, new_name: String) -> Result<(), CategoryGenericError> {
        if new_name.is_empty() {
            return Err(CategoryGenericError::NameIsEmpty());
        }
        self.name = new_name;
        Ok(())
    }

    pub fn change_description(&mut self, new_description: String) -> Result<(), CategoryGenericError> {
        if new_description.is_empty() {
            return Err(CategoryGenericError::DescriptionIsEmpty());
        }
        self.description = new_description;
        Ok(())
    }

    pub fn change_auth(&mut self, new_auth: bool) {
        self.auth = new_auth;
    }

    pub fn get_rule_table(&self) -> &RuleTableEntity {
        &self.rule_table
    }

    pub fn get_mut_rule_table(&mut self) -> &mut RuleTableEntity {
        &mut self.rule_table
    }
}
