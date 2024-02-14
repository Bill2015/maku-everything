use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ResourceAddTagDto {
    pub id: String,

    pub tag_id: String,
}
