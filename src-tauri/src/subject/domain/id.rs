use serde::Serialize;
use crate::common::domain::ID;
use crate::impl_domain_id;

#[derive(Debug, Serialize, Clone)]
pub struct SubjectID(String);

impl_domain_id!(SubjectID);
