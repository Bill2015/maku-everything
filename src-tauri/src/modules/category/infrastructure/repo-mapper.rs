use surrealdb::sql::thing;
use surrealdb::sql::Datetime;

use crate::modules::common::domain::ID;
use crate::modules::common::infrastructure::IRepoMapper;
use crate::modules::category::domain::{CategoryAggregate, CategoryID};
use crate::modules::category::repository::CategoryDO;

// Mapper
pub struct CategoryRepoMapper {}
impl IRepoMapper<CategoryAggregate, CategoryDO> for CategoryRepoMapper {
    fn do_to_aggregate(category_do: CategoryDO) -> CategoryAggregate {
        CategoryAggregate {
            id: CategoryID::from(category_do.id.to_string()),
            name: category_do.name,
            description: category_do.description,
            root_path: category_do.root_path,
            auth: category_do.auth,
            created_at: category_do.created_at.0,
            updated_at: category_do.updated_at.0,
        }
    }
    
    fn aggregate_to_do(aggregate: CategoryAggregate) -> CategoryDO {
        CategoryDO {
            id: thing(aggregate.id.to_str()).unwrap(),
            name: aggregate.name,
            description: aggregate.description,
            root_path: aggregate.root_path,
            auth: aggregate.auth,
            created_at: Datetime(aggregate.created_at),
            updated_at: Datetime(aggregate.updated_at),
        }
    }
}