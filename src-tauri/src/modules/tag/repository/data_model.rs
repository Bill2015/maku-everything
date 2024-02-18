use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "tag_type", content = "attr")]
pub enum TagAttrDO {
    #[serde(rename = "normal")]
    Normal,

    #[serde(rename = "number")]
    WithNumber {
        start: i64,
        end: i64,
        defval: i64,
    },

    #[serde(rename = "text")]
    WithText { defval: String },

    #[serde(rename = "date")]
    WithDate { defval: Datetime },

    #[serde(rename = "bool")]
    WithBool { defval: bool },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagDO {
   pub id: Thing,

   pub name: String,

   pub description: String,

   pub auth: bool,

   pub created_at: Datetime,

   pub updated_at: Datetime,

   pub belong_category: Thing, 

   pub belong_subject: Thing,

   #[serde(flatten)]
   pub attrval: TagAttrDO,
}
