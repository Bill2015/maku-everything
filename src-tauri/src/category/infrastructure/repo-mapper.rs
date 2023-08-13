use surrealdb::sql::Thing;

use crate::common::domain::ID;
use crate::common::infrastructure::IRepoMapper;
use crate::category::domain::{CategoryAggregate, CategoryID};
use crate::category::repository::CategoryDO;
use crate::category::repository::CATEGORY_DB_NAMESPACE;

// Mapper
pub struct CategoryRepoMapper {}
impl IRepoMapper<CategoryAggregate, CategoryDO> for CategoryRepoMapper {
    fn do_to_aggregate(category_do: CategoryDO) -> CategoryAggregate {
        CategoryAggregate {
            id: CategoryID::parse(category_do.id.unwrap().to_string()),
            title: category_do.title,
            description: category_do.description,
            auth: category_do.auth,
            created_at: category_do.created_at,
            updated_at: category_do.updated_at,
        }
    }
    
    fn aggregate_to_do(aggregate: CategoryAggregate) -> CategoryDO {
        CategoryDO {
            id: Some(Thing::from((String::from(CATEGORY_DB_NAMESPACE), aggregate.id.to_string()))),
            title: aggregate.title,
            description: aggregate.description,
            auth: aggregate.auth,
            created_at: aggregate.created_at,
            updated_at: aggregate.updated_at,
        }
    }
}