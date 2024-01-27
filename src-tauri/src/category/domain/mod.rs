use std::path::Path;

use chrono::NaiveDateTime;
use serde::Serialize;
use chrono::{DateTime, Utc};

use crate::common::domain::Porting;
use crate::common::infrastructure::date;
use crate::common::domain::ID;

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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CategoryAggregate {
    fn relove_path(path: String) -> Result<String, CategoryGenericError> {
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
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }
        )
    }

    pub fn set_updated_at(&mut self, new_date: &str) -> Result<(), CategoryGenericError> {
        if let Ok(date) = NaiveDateTime::parse_from_str(new_date, date::DATE_TIME_FORMAT) {
            self.updated_at = date.and_utc();
            return Ok(())
        }
        Err(CategoryGenericError::InvalidDateFormat())
    }

    pub fn set_created_at(&mut self, new_date: &str) -> Result<(), CategoryGenericError> {
        if let Ok(date) = NaiveDateTime::parse_from_str(new_date, date::DATE_TIME_FORMAT) {
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
}

impl Porting<PortingCategoryObject> for CategoryAggregate {
    type Err = CategoryGenericError;
    fn import_from(data: PortingCategoryObject) -> Result<Self, Self::Err> {
        let new_path = Self::relove_path(data.root_path)?;
        let mut category = Self::new(data.name, data.description, new_path)?;
        category.set_created_at(&data.created_at)?;
        category.set_updated_at(&data.updated_at)?;
        Ok(category)
    }

    fn export_to(self) -> Result<PortingCategoryObject, Self::Err> {
        Ok(PortingCategoryObject {
            id: self.id,
            name: self.name,
            description: self.description,
            root_path: self.root_path,
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
            auth: self.auth,
        })
    }
}

