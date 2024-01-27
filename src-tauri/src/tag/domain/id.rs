use serde::{Serialize, Deserialize};
use surrealdb::sql::Id;
use crate::common::domain::ID;
use crate::common::repository::tablens;
use crate::impl_domain_id;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TagID(String);

impl_domain_id!(TagID, tablens::TAG);
