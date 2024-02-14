use chrono::Utc;

use crate::modules::category::domain::CategoryID;
use crate::modules::common::domain::ID;
use crate::modules::common::infrastructure::dateutils;

use super::{Category, CategoryGenericError, CategoryPlainObject, CategoryProps, CategoryMapperRuleItemVO, CategoryMapperRuleEntity};

pub struct CategoryFactory { }

impl CategoryFactory {
    pub fn create(name: String, description: String, root_path: String) -> Result<Category, CategoryGenericError> {
        let new_path = Category::relove_path(root_path)?;

        // name can't be empty
        if name.len() <= 0 {
            return Err(CategoryGenericError::NameIsEmpty());
        }

        Ok(Category::new(CategoryProps {
            id: CategoryID::new(),
            name: name,
            description: description,
            root_path: new_path,
            auth: false,
            rule_table: CategoryMapperRuleEntity::new(Vec::new()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }))
    }

    pub fn reconstitute(properties: CategoryProps) -> Category {
        Category::new(properties)
    }

    pub fn from_plain(object: CategoryPlainObject) -> Result<Category, CategoryGenericError> {
        let new_path = Category::relove_path(object.root_path)?;

        let rules = object.rules
            .into_iter()
            .map(|x| CategoryMapperRuleItemVO { text: x.text, tag_id: x.tag_id })
            .collect();

        Ok(Category::new(CategoryProps {
            id: CategoryID::new(),
            name: object.name,
            description: object.description,
            root_path: new_path,
            auth: object.auth,
            rule_table: CategoryMapperRuleEntity::new(rules),
            created_at: dateutils::parse(&object.created_at)
                .map_err(|_| CategoryGenericError::InvalidDateFormat())?
                .and_utc(),
            updated_at: dateutils::parse(&object.updated_at)
                .map_err(|_| CategoryGenericError::InvalidDateFormat())?
                .and_utc(),
        }))
    }
}
