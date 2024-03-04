use serde::{Deserialize, Serialize};


/**
 * counting Data Object */
 #[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CountDO {
    pub count: i64,
}
