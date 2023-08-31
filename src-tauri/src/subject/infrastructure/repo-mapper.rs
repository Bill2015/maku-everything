use surrealdb::sql::{Thing, thing};
use surrealdb::sql::Datetime;

use crate::common::repository::tablens;
use crate::subject::domain::{SubjectAggregate, SubjectID};
use crate::category::domain::CategoryID;
use crate::common::infrastructure::IRepoMapper;
use crate::common::domain::ID;
use crate::subject::repository::SubjectDO;

// Mapper
pub struct SubjectRepoMapper {}
impl IRepoMapper<SubjectAggregate, SubjectDO> for SubjectRepoMapper {
    fn do_to_aggregate(subject_do: SubjectDO) -> SubjectAggregate {
        SubjectAggregate {
            id: SubjectID::from(subject_do.id.to_string()),
            name: subject_do.name,
            description: subject_do.description,
            belong_category: CategoryID::from(subject_do.belong_category.to_string()),
            auth: subject_do.auth,
            created_at: subject_do.created_at.0,
            updated_at: subject_do.updated_at.0,
        }
    }
    
    fn aggregate_to_do(aggregate: SubjectAggregate) -> SubjectDO {
        let id = match thing(aggregate.id.to_str()) {
            Ok(value) => value,
            _ => Thing::from((tablens::SUBJECT, ""))
        };
        SubjectDO {
            id: id,
            name: aggregate.name,
            description: aggregate.description,
            belong_category: aggregate.belong_category.to_string(),
            auth: aggregate.auth,
            created_at: Datetime(aggregate.created_at),
            updated_at: Datetime(aggregate.updated_at),
        }
    }
}