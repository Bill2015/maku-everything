use surrealdb::sql::Thing;

use crate::tag::domain::{TagAggregate, TagID};
use crate::common::infrastructure::IRepoMapper;
use crate::common::domain::ID;
use crate::tag::repository::{TagDO, TAG_DB_NAMESPACE};

// Mapper
pub struct TagRepoMapper {}
impl IRepoMapper<TagAggregate, TagDO> for TagRepoMapper {
    fn do_to_aggregate(tag_do: TagDO) -> TagAggregate {
        TagAggregate {
            id: TagID::parse(tag_do.id.unwrap().id.to_string()),
            name: tag_do.name,
            description: tag_do.description,
            belong_category: tag_do.belong_category,
            auth: tag_do.auth,
            created_at: tag_do.created_at,
            updated_at: tag_do.updated_at,
        }
    }
    
    fn aggregate_to_do(aggregate: TagAggregate) -> TagDO {
        TagDO {
            id:  Some(Thing::from((String::from(TAG_DB_NAMESPACE), aggregate.id.to_string()))),
            name: aggregate.name,
            description: aggregate.description,
            belong_category: aggregate.belong_category,
            auth: aggregate.auth,
            created_at: aggregate.created_at,
            updated_at: aggregate.updated_at,
        }
    }
}