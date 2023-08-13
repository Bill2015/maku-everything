use surrealdb::sql::Thing;

use crate::subject::domain::{SubjectAggregate, SubjectID};
use crate::category::domain::CategoryID;
use crate::category::repository::CATEGORY_DB_NAMESPACE;
use crate::common::infrastructure::IRepoMapper;
use crate::common::domain::ID;
use crate::subject::repository::{SubjectDO, SUBJECT_DB_NAMESPACE};

// Mapper
pub struct SubjectRepoMapper {}
impl IRepoMapper<SubjectAggregate, SubjectDO> for SubjectRepoMapper {
    fn do_to_aggregate(subject_do: SubjectDO) -> SubjectAggregate {
        SubjectAggregate {
            id: SubjectID::parse(subject_do.id.unwrap().id.to_string()),
            name: subject_do.name,
            description: subject_do.description,
            belong_category: CategoryID::parse(subject_do.belong_category.id.to_string()),
            auth: subject_do.auth,
            created_at: subject_do.created_at,
            updated_at: subject_do.updated_at,
        }
    }
    
    fn aggregate_to_do(aggregate: SubjectAggregate) -> SubjectDO {
        SubjectDO {
            id:  Some(Thing::from((String::from(SUBJECT_DB_NAMESPACE), aggregate.id.to_string()))),
            name: aggregate.name,
            description: aggregate.description,
            belong_category: Thing::from((String::from(CATEGORY_DB_NAMESPACE), aggregate.belong_category.to_string())),
            auth: aggregate.auth,
            created_at: aggregate.created_at,
            updated_at: aggregate.updated_at,
        }
    }
}