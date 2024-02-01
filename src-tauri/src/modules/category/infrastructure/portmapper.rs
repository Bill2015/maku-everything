use crate::modules::common::domain::{Porting, ID};
use crate::modules::category::domain::{CategoryAggregate, CategoryGenericError, CategoryID, PortingCategoryObject};
use crate::modules::common::infrastructure::dateutils;

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
            created_at: dateutils::parse(&data.created_at)
                .map_err(|_| CategoryGenericError::InvalidDateFormat())?
                .and_utc(),
            updated_at: dateutils::parse(&data.updated_at)
                .map_err(|_| CategoryGenericError::InvalidDateFormat())?
                .and_utc(),
        };
        
        Ok(category)
    }

    fn export_to(self) -> Result<PortingCategoryObject, Self::Err> {
        Ok(PortingCategoryObject {
            id: self.id,
            name: self.name,
            description: self.description,
            root_path: self.root_path,
            created_at: dateutils::format(self.created_at),
            updated_at: dateutils::format(self.updated_at),
            auth: self.auth,
        })
    }
}

