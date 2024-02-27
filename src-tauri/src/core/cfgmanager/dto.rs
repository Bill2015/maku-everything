use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ChangeConfigDto {
    pub lang: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigDto {
   pub lang: String
}
