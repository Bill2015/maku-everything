use std::{path::Path};
use serde::Serialize;
use crate::impl_domain_id;
use crate::common::domain::ID;

#[derive(Debug, Serialize)]
pub struct TagID {
    pub id: String,
}

impl TagID {
    fn new(s: String) -> Self {
        TagID { id: s }
    }
}

impl_domain_id!(TagID);
