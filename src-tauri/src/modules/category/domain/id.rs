use serde::{Serialize, Deserialize};
use surrealdb::sql::Id;
use crate::modules::common::domain::ID;
use crate::modules::common::repository::tablens;
use crate::impl_domain_id;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct CategoryID(String);

impl_domain_id!(CategoryID, tablens::CATEGORY);
