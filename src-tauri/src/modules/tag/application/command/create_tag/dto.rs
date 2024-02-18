use serde::{Serialize, Deserialize};

// Quick Example: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=15cfab66d38ff8a15a9cf1d8d897ac68
// See also: https://serde.rs/enum-representations.html
#[derive(Serialize, Deserialize)]
#[serde(tag = "tag_type", content = "attr")]
pub enum CreateTagTypeDto {
    #[serde(rename = "normal")]
    Normal,

    #[serde(rename = "number")]
    Number { start: i64, end: i64, defval: i64 },

    #[serde(rename = "text")]
    Text { defval: String },

    #[serde(rename = "date")]
    Date { defval: String },

    #[serde(rename = "bool")]
    Bool { defval: bool },
}

#[derive(Serialize, Deserialize)]
pub struct CreateTagDto {
    pub name: String,

    pub description: String,

    pub belong_category: String,

    pub belong_subject: String,

    #[serde(flatten)]
    pub attrval: CreateTagTypeDto,
}
