use surrealdb::sql::{Thing, thing};
use surrealdb::sql::Datetime;

use crate::category::domain::CategoryID;
use crate::common::repository::tablens;
use crate::subject::domain::SubjectID;
use crate::tag::domain::{TagAggregate, TagID};
use crate::common::infrastructure::IRepoMapper;
use crate::common::domain::ID;
use crate::tag::repository::TagDO;

// Mapper
pub struct TagRepoMapper {}
impl IRepoMapper<TagAggregate, TagDO> for TagRepoMapper {
    fn do_to_aggregate(tag_do: TagDO) -> TagAggregate {
        TagAggregate {
            id: TagID::from(tag_do.id.to_string()),
            name: tag_do.name,
            description: tag_do.description,
            belong_category: CategoryID::from(tag_do.belong_category),
            belong_subject: SubjectID::from(tag_do.belong_subject),
            auth: tag_do.auth,
            created_at: tag_do.created_at.0,
            updated_at: tag_do.updated_at.0,
        }
    }
    
    fn aggregate_to_do(aggregate: TagAggregate) -> TagDO {
        let id = match thing(aggregate.id.to_str()) {
            Ok(value) => value,
            _ => Thing::from((tablens::TAG, ""))
        };
        TagDO {
            id:  id,
            name: aggregate.name,
            description: aggregate.description,
            belong_category: aggregate.belong_category.to_string(),
            belong_subject: aggregate.belong_subject.to_string(),
            auth: aggregate.auth,
            created_at: Datetime(aggregate.created_at),
            updated_at: Datetime(aggregate.updated_at),
        }
    }
}