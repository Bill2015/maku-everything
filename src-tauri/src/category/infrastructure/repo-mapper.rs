use surrealdb::sql::{Thing, thing};
use surrealdb::sql::Datetime;
use chrono::{DateTime, Utc};

use crate::common::domain::ID;
use crate::common::infrastructure::IRepoMapper;
use crate::common::repository::tablens;
use crate::category::domain::{CategoryAggregate, CategoryID};
use crate::category::repository::CategoryDO;

// Mapper
pub struct CategoryRepoMapper {}
impl IRepoMapper<CategoryAggregate, CategoryDO> for CategoryRepoMapper {
    fn do_to_aggregate(category_do: CategoryDO) -> CategoryAggregate {
        CategoryAggregate {
            id: CategoryID::parse(category_do.id.to_string()),
            title: category_do.title,
            description: category_do.description,
            auth: category_do.auth,
            created_at: category_do.created_at.0,
            updated_at: category_do.updated_at.0,
        }
    }
    
    fn aggregate_to_do(aggregate: CategoryAggregate) -> CategoryDO {
        let id = match thing(aggregate.id.to_str()) {
            Ok(value) => value,
            _ => Thing::from((tablens::CATEGORY, ""))
        };
        CategoryDO {
            id: id,
            title: aggregate.title,
            description: aggregate.description,
            auth: aggregate.auth,
            created_at: Datetime(aggregate.created_at),
            updated_at: Datetime(aggregate.updated_at),
        }
    }
}