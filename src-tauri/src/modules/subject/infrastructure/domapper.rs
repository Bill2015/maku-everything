use surrealdb::sql::Datetime;

use crate::modules::subject::domain::SubjectAggregate;
use crate::modules::subject::repository::SubjectDO;

impl From<SubjectDO> for SubjectAggregate {
    fn from(value: SubjectDO) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
            description: value.description,
            belong_category: value.belong_category.into(),
            auth: value.auth,
            created_at: value.created_at.0,
            updated_at: value.created_at.0,
        }
    }
}
impl Into<SubjectDO> for SubjectAggregate {
    fn into(self) -> SubjectDO {
        SubjectDO {
            id: self.id.into(),
            name: self.name,
            description: self.description,
            belong_category: self.belong_category.into(),
            auth: self.auth,
            created_at: Datetime(self.created_at),
            updated_at: Datetime(self.updated_at),
        }
    }
}