use serde::Serialize;
use surrealdb::sql::Id;
use crate::common::domain::ID;
use crate::common::repository::tablens;
use crate::impl_domain_id;

#[derive(Debug, Serialize, Clone)]
pub struct CategoryID(String);

impl_domain_id!(CategoryID, tablens::CATEGORY);
