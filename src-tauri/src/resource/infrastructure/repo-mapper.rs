use surrealdb::sql::Datetime;
use surrealdb::sql::Thing;
use surrealdb::sql::thing;

use crate::category::domain::CategoryID;
use crate::resource::domain::ResourceFileAggregate;
use crate::resource::domain::{ResourceAggregate, ResourceID};
use crate::resource::repository::ResourceFileDo;
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

        let file = match resource_do.file {
            Some(value) => Some(ResourceFileAggregate::from_do(
                value.uuid, 
                value.name, 
                value.path,
                value.ext, 
            )),
            None => None,
        };

        ResourceAggregate {
            id: ResourceID::from(resource_do.id.to_string()),
            name: resource_do.name,
            description: resource_do.description,
            belong_category: CategoryID::from(resource_do.belong_category.to_string()),
            file: file,
            auth: resource_do.auth,
            tags: tags,
            new_tags: Vec::new(),
            del_tags: Vec::new(),
            created_at: resource_do.created_at.0,
            updated_at: resource_do.updated_at.0,
        }
    }
    
    fn aggregate_to_do(aggregate: ResourceAggregate) -> ResourceDO {
        let tags: Vec<Thing> = aggregate.tags
            .iter()
            .map(|x| thing(x.to_str()).unwrap())
            .collect();

        let id = thing(aggregate.id.to_str())
            .unwrap_or(Thing::from((tablens::RESOURCE, "")));

        let belong_category = thing(aggregate.belong_category.to_str())
            .unwrap_or(Thing::from((tablens::CATEGORY, "")));

        let file = match aggregate.file {
            Some(value) => Some(ResourceFileDo {
                uuid: value.uuid,
                name: value.name,
                path: value.path,
                ext: value.ext,
            }),
            None => None,
        };

        ResourceDO {
            id: id,
            name: aggregate.name,
            description: aggregate.description,
            belong_category: belong_category,
            file: file,
            auth: aggregate.auth,
            tags: tags,
            created_at: Datetime(aggregate.created_at),
            updated_at: Datetime(aggregate.updated_at),
        }
    }
}