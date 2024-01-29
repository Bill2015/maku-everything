use surrealdb::sql::Datetime;
use surrealdb::sql::Thing;
use surrealdb::sql::thing;

use crate::modules::category::domain::CategoryID;
use crate::modules::resource::domain::ResourceFileAggregate;
use crate::modules::resource::domain::ResourceUrlAggregate;
use crate::modules::resource::domain::{ResourceAggregate, ResourceID};
use crate::modules::resource::repository::ResourceFileDo;
use crate::modules::resource::repository::ResourceUrlDo;
use crate::modules::tag::domain::TagID;
use crate::modules::common::domain::ID;
use crate::modules::common::infrastructure::IRepoMapper;
use crate::modules::resource::repository::ResourceDO;

// Mapper
pub struct ResourceRepoMapper {}
impl IRepoMapper<ResourceAggregate, ResourceDO> for ResourceRepoMapper {
    fn do_to_aggregate(resource_do: ResourceDO) -> ResourceAggregate {
        let tags: Vec<TagID> = resource_do.tags
            .iter()
            .map(|x| TagID::from(x.to_string()))
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

        let url = match resource_do.url {
            Some(value) => Some(ResourceUrlAggregate::from_do(
                value.host,
                value.full,
            )),
            None => None,
        };

        ResourceAggregate {
            id: ResourceID::from(resource_do.id.to_string()),
            name: resource_do.name,
            description: resource_do.description,
            root_path: resource_do.root_path,
            belong_category: CategoryID::from(resource_do.belong_category.to_string()),
            file: file,
            url: url,
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

        let file = match aggregate.file {
            Some(value) => Some(ResourceFileDo {
                uuid: value.uuid,
                name: value.name,
                path: value.path,
                ext: value.ext,
            }),
            None => None,
        };

        let url = match aggregate.url {
            Some(value) => Some(ResourceUrlDo {
                host: value.host,
                full: value.full,
            }),
            None => None,
        };

        ResourceDO {
            id: thing(aggregate.id.to_str()).unwrap(),
            name: aggregate.name,
            description: aggregate.description,
            root_path: aggregate.root_path,
            belong_category: thing(aggregate.belong_category.to_str()).unwrap(),
            file: file,
            url: url,
            auth: aggregate.auth,
            tags: tags,
            created_at: Datetime(aggregate.created_at),
            updated_at: Datetime(aggregate.updated_at),
        }
    }
}