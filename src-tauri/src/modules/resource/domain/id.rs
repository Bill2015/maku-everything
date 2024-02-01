use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing, thing};
use crate::modules::common::domain::ID;
use crate::modules::common::repository::tablens;
use crate::impl_domain_id;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResourceID(String);

impl_domain_id!(ResourceID, tablens::RESOURCE);

