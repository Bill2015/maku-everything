use surrealdb::sql::Datetime;

use crate::modules::tag::domain::TagAggregate;
use crate::modules::tag::repository::TagDO;

impl From<TagDO> for TagAggregate {
    fn from(value: TagDO) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
            description: value.description,
            belong_category: value.belong_category.into(),
            belong_subject: value.belong_subject.into(),
            auth: value.auth,
            created_at: value.created_at.0,
            updated_at: value.updated_at.0,
        }
    }
}
impl Into<TagDO> for TagAggregate {
    fn into(self) -> TagDO {
        TagDO {
            id: self.id.into(),
            name: self.name,
            description: self.description,
            belong_category: self.belong_category.into(),
            belong_subject: self.belong_subject.into(),
            auth: self.auth,
            created_at: Datetime(self.created_at),
            updated_at: Datetime(self.updated_at),
        }
    }
}