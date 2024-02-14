use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ResourceRemoveTagDto {
    pub id: String,

    pub tag_id: String,
}
