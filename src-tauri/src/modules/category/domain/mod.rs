use std::path::Path;

use serde::Serialize;
use chrono::{DateTime, Utc};

use crate::base_aggregate;
use crate::modules::common::domain::ToPlainObject;
use crate::modules::common::infrastructure::dateutils;

mod entities;
pub use entities::CategoryAddRuleEntity;

mod valueobj;
pub use valueobj::CategoryAddRuleItemVO;

mod error;
pub use error::CategoryGenericError;
pub use error::CategoryError;

mod id;
pub use id::CategoryID;

mod factory;
pub use factory::CategoryFactory;

mod plainobj;
pub use plainobj::{CategoryPlainObject, CategoryAddRulePlainObject};

base_aggregate!(Category {
    id: CategoryID,
    name: String,
    description: String,
    root_path: String,
    auth: bool,
    rule_table: CategoryAddRuleEntity,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
});

impl Category {
    pub fn take_id(self) -> CategoryID {
        self.id
    }

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

    pub fn get_mut_rule_table(&mut self) -> &mut CategoryAddRuleEntity {
        &mut self.rule_table
    }
}

impl ToPlainObject<CategoryPlainObject> for Category {
    fn to_plain(self) -> CategoryPlainObject {
        let rules = self.rule_table
            .take_rules()
            .into_iter()
            .map(|x| CategoryAddRulePlainObject { text: x.text, tag_id: x.tag_id })
            .collect();

        CategoryPlainObject {
            id: self.id,
            name: self.name,
            description: self.description,
            root_path: self.root_path,
            rules: rules,
            created_at: dateutils::format(self.created_at),
            updated_at: dateutils::format(self.updated_at),
            auth: self.auth,
        }
    }
}
