use std::{path::Path};
use serde::Serialize;
use serde::Serializer;

use crate::impl_domain_id;
use crate::common::domain::ID;
use crate::category::domain::CategoryID;


#[derive(Debug, Serialize)]
pub struct SubjectID {
    id: String,
}

impl SubjectID {
    fn new(s: String) -> Self {
        SubjectID { id: s }
    }
}

impl_domain_id!(SubjectID);
