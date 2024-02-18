use serde::{Deserialize, Deserializer, Serialize};
use surrealdb::sql::{Datetime, Thing, Value};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceFileDo {
    pub uuid: String,
    pub name: String,
    pub path: String,
    pub ext: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceUrlDo {
    pub host: String,
    pub full: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, strum_macros::Display)]
#[serde(tag = "tagging_type", content = "attrval")]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ResourceTaggingAttrDO {
    #[serde(deserialize_with = "ResourceTaggingAttrDO::deserialize_null_default")]
    Normal(),

    Number(i64),

    Text(String),

    Date(Datetime),

    Bool(bool),
}
impl ResourceTaggingAttrDO {
    fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        T: Default + Deserialize<'de>,
        D: Deserializer<'de>,
    {
        let opt = Option::deserialize(deserializer)?;
        Ok(opt.unwrap_or_default())
    }
}
impl Into<Value> for ResourceTaggingAttrDO {
    fn into(self) -> Value {    
        match self {
            Self::Normal() => Value::Null,
            Self::Number(val) => Value::Number(val.into()),
            Self::Text(val) => Value::Strand(val.into()),
            Self::Date(val) => Value::Datetime(val.into()),
            Self::Bool(val) => Value::Bool(val.into()),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceTaggingDo {
    #[serde(alias = "in")]
    pub id: Thing,

    pub added_at: Datetime,

    #[serde(flatten)]
    pub attrval: ResourceTaggingAttrDO,
}

/**
 * Resource Data Object */
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceDO {
    pub id: Thing,
    pub name: String,
    pub description: String,
    pub file: Option<ResourceFileDo>,
    pub url: Option<ResourceUrlDo>,
    pub auth: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub belong_category: Thing,
    
    #[serde(skip_serializing)]
    #[serde(default = "String::default")]
    pub root_path: String,

    #[serde(skip_serializing)]
    #[serde(default = "Vec::new")]
    pub tags: Vec<ResourceTaggingDo>,
}
