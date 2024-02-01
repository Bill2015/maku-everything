use chrono::NaiveDateTime;

use crate::modules::common::domain::{Porting, ID};
use crate::modules::category::domain::{CategoryAggregate, CategoryGenericError, CategoryID, PortingCategoryObject};
use crate::modules::common::infrastructure::date;

impl Porting<PortingCategoryObject> for CategoryAggregate {
    type Err = CategoryGenericError;
    fn import_from(data: PortingCategoryObject) -> Result<Self, Self::Err> {
        let new_path = Self::relove_path(data.root_path)?;
        let category = Self {
            id: CategoryID::new(),
            name: data.name,
            description: data.description,
            root_path: new_path,
            auth: data.auth,
            created_at: NaiveDateTime::parse_from_str(&data.created_at, date::DATE_TIME_FORMAT).unwrap().and_utc(),
            updated_at: NaiveDateTime::parse_from_str(&data.updated_at, date::DATE_TIME_FORMAT).unwrap().and_utc(),
        };
        
        Ok(category)
    }

    fn export_to(self) -> Result<PortingCategoryObject, Self::Err> {
        Ok(PortingCategoryObject {
            id: self.id,
            name: self.name,
            description: self.description,
            root_path: self.root_path,
            created_at: self.created_at.format(date::DATE_TIME_FORMAT).to_string(),
            updated_at: self.updated_at.format(date::DATE_TIME_FORMAT).to_string(),
            auth: self.auth,
        })
    }
}

