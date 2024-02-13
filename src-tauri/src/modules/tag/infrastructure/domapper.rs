use surrealdb::sql::Datetime;

use crate::modules::common::domain::DomainModelMapper;
use crate::modules::tag::domain::TagProps;
use crate::modules::tag::repository::TagDO;

impl DomainModelMapper<TagProps> for TagDO {
    fn to_domain(self) -> TagProps {
        TagProps {
            id: self.id.into(),
            name: self.name,
            description: self.description,
            belong_category: self.belong_category.into(),
            belong_subject: self.belong_subject.into(),
            auth: self.auth,
            created_at: self.created_at.0,
            updated_at: self.updated_at.0,
        }
    }
    fn from_domain(value: TagProps) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
            description: value.description,
            belong_category: value.belong_category.into(),
            belong_subject: value.belong_subject.into(),
            auth: value.auth,
            created_at: Datetime(value.created_at),
            updated_at: Datetime(value.updated_at),
        }
    }
}

