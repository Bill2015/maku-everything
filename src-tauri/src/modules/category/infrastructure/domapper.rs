use surrealdb::sql::Datetime;

use crate::modules::category::domain::CategoryAggregate;
use crate::modules::category::repository::CategoryDO;

impl From<CategoryDO> for CategoryAggregate {
    fn from(value: CategoryDO) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
            description: value.description,
            root_path: value.root_path,
            auth: value.auth,
            created_at: value.created_at.0,
            updated_at: value.created_at.0,
        }
    }
}
impl Into<CategoryDO> for CategoryAggregate {
    fn into(self) -> CategoryDO {
        CategoryDO {
            id: self.id.into(),
            name: self.name,
            description: self.description,
            root_path: self.root_path,
            auth: self.auth,
            created_at: Datetime(self.created_at),
            updated_at: Datetime(self.updated_at),
        }
    }
}