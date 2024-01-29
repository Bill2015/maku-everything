use serde::{Deserialize, Serialize};
use surrealdb::sql::Id;
use crate::modules::common::domain::ID;
use crate::modules::common::repository::tablens;
use crate::impl_domain_id;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubjectID(String);

impl_domain_id!(SubjectID, tablens::SUBJECT);
