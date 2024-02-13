use surrealdb::sql::Datetime;

use crate::modules::common::domain::DomainModelMapper;
use crate::modules::subject::domain::SubjectProps;
use crate::modules::subject::repository::SubjectDO;

impl DomainModelMapper<SubjectProps> for SubjectDO {
    fn to_domain(self) -> SubjectProps {
        SubjectProps {
            id: self.id.into(),
            name: self.name,
            description: self.description,
            belong_category: self.belong_category.into(),
            auth: self.auth,
            created_at: self.created_at.0,
            updated_at: self.created_at.0,
        }
    }
    fn from_domain(value: SubjectProps) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
            description: value.description,
            belong_category: value.belong_category.into(),
            auth: value.auth,
            created_at: Datetime(value.created_at),
            updated_at: Datetime(value.updated_at),
        }
    }
}
