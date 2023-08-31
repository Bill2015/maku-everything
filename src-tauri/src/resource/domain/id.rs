use serde::Serialize;
use crate::impl_domain_id;
use crate::common::domain::ID;

#[derive(Debug, Serialize, Clone)]
pub struct ResourceID {
    id: String,
}

impl ResourceID {
    fn new(s: String) -> Self {
        ResourceID { id: s }
    }
}


impl_domain_id!(ResourceID);