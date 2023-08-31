use serde::Serialize;
use crate::impl_domain_id;
use crate::common::domain::ID;

#[derive(Debug, Serialize, Clone)]
pub struct CategoryID {
    id: String,
}

impl_domain_id!(CategoryID);
