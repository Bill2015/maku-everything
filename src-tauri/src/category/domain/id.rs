use serde::Serialize;
use crate::impl_domain_id;
use crate::common::domain::ID;

#[derive(Debug, Serialize)]
pub struct CategoryID {
    id: String,
}

impl CategoryID {
    fn new(s: String) -> Self {
        CategoryID { id: s }
    }
}


impl_domain_id!(CategoryID);
