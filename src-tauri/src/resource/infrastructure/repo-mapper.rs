use surrealdb::sql::Thing;
use surrealdb::sql::thing;

use crate::category::domain::CategoryID;
use crate::resource::domain::{ResourceAggregate, ResourceID};
use crate::tag::domain::TagID;
use crate::common::domain::ID;
use crate::common::repository::tablens;
use crate::common::infrastructure::IRepoMapper;
use crate::resource::repository::ResourceDO;

// Mapper
pub struct ResourceRepoMapper {}
impl IRepoMapper<ResourceAggregate, ResourceDO> for ResourceRepoMapper {
    fn do_to_aggregate(resource_do: ResourceDO) -> ResourceAggregate {
        let tags: Vec<TagID> = resource_do.tags
            .iter()
            .map(|x|  TagID { id: x.to_string() })
            .collect();
        ResourceAggregate {
            id: ResourceID::parse(resource_do.id.to_string()),
            title: resource_do.title,
            description: resource_do.description,
            belong_category: CategoryID::parse(resource_do.belong_category),
            file_id: resource_do.file_id,
            file_name: resource_do.file_name,
            file_path: resource_do.file_path,
            file_type: resource_do.file_type,
            auth: resource_do.auth,
            tags: tags,
            created_at: resource_do.created_at,
            updated_at: resource_do.updated_at,
        }
    }
    
    fn aggregate_to_do(aggregate: ResourceAggregate) -> ResourceDO {
        let tags: Vec<String> = aggregate.tags
            .iter()
            .map(|x| x.id.clone())
            .collect();
        let id = match thing(aggregate.id.to_str()) {
            Ok(value) => value,
            _ => Thing::from((tablens::RESOURCE, ""))
        };
        ResourceDO {
            id: id,
            title: aggregate.title,
            description: aggregate.description,
            belong_category: aggregate.belong_category.to_string(),
            file_id: aggregate.file_id,
            file_name: aggregate.file_name,
            file_path: aggregate.file_path,
            file_type: aggregate.file_type,
            auth: aggregate.auth,
            tags: tags,
            created_at: aggregate.created_at,
            updated_at: aggregate.updated_at,
        }
    }
}