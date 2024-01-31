use surrealdb::sql::thing;
use surrealdb::sql::Datetime;

use crate::modules::category::domain::CategoryID;
use crate::modules::subject::domain::SubjectID;
use crate::modules::tag::domain::{TagAggregate, TagID};
use crate::modules::common::infrastructure::IRepoMapper;
use crate::modules::common::domain::ID;
use crate::modules::tag::repository::TagDO;

// Mapper
pub struct TagRepoMapper {}
impl IRepoMapper<TagAggregate, TagDO> for TagRepoMapper {
    fn do_to_aggregate(tag_do: TagDO) -> TagAggregate {
        TagAggregate {
            id: TagID::from(tag_do.id.to_string()),
            name: tag_do.name,
            description: tag_do.description,
            belong_category: CategoryID::from(tag_do.belong_category.to_string()),
            belong_subject: SubjectID::from(tag_do.belong_subject.to_string()),
            auth: tag_do.auth,
            created_at: tag_do.created_at.0,
            updated_at: tag_do.updated_at.0,
        }
    }
    
    fn aggregate_to_do(aggregate: TagAggregate) -> TagDO {
        TagDO {
            id: thing(aggregate.id.to_str()).unwrap(),
            name: aggregate.name,
            description: aggregate.description,
            belong_category: thing(aggregate.belong_category.to_str()).unwrap(),
            belong_subject: thing(aggregate.belong_subject.to_str()).unwrap(),
            auth: aggregate.auth,
            created_at: Datetime(aggregate.created_at),
            updated_at: Datetime(aggregate.updated_at),
        }
    }
}