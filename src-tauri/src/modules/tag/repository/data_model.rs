use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "tag_type", content = "attr")]
#[serde(rename_all = "snake_case")]
pub enum TagAttrDO {
    Normal,

    Number {
        start: i64,
        end: i64,
        defval: i64,
    },

    Text { defval: String },

    Date { defval: Datetime },

    Bool { defval: bool },
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
