use chrono::NaiveDate;

use crate::modules::common::repository::sql_utils;

use super::{AttributeValue, AttributeValueType};


///
/// System Pre-defined Tag
pub enum SystemTag {
    URL(Option<String>),
    TagNums((Option<usize>, Option<usize>)),
    File(Option<String>),
    FileExt(String),
    Name(String),
    CreatedAt((Option<NaiveDate>, Option<NaiveDate>)),
    UpdatedAt((Option<NaiveDate>, Option<NaiveDate>)),
}

impl SystemTag {
    const TAG_URL: &str = "@maku:url";
    const TAG_TAGNUM: &str = "@maku:tagnum";
    const TAG_FILE: &str = "@maku:file";
    const TAG_FILEXT: &str = "@maku:filext";
    const TAG_NAME: &str = "@maku:name";
    const TAG_CREATED: &str = "@maku:created";
    const TAG_UPDATED: &str = "@maku:updated";

    pub fn full_name(namespace: &str, val: &str) -> String {
        format!("{}:{}", namespace, val)
    }

    pub fn is_defined<S: Into<String>>(val: S) -> bool {
        match val.into().to_lowercase().as_str() {
            Self::TAG_URL => true,
            Self::TAG_TAGNUM => true,
            Self::TAG_FILE => true,
            Self::TAG_FILEXT => true,
            Self::TAG_NAME => true,
            Self::TAG_CREATED => true,
            Self::TAG_UPDATED => true,
            _ => false,
        }
    }

    pub fn attr_type<S: Into<String>>(namespace: S, name: S) -> Result<AttributeValueType, String> {
        let s = format!("{}:{}", namespace.into(), name.into());
        match s.to_lowercase().as_str() {
            Self::TAG_URL => Ok(AttributeValueType::OptionText),
            Self::TAG_TAGNUM => Ok(AttributeValueType::NumberRange),
            Self::TAG_FILE => Ok(AttributeValueType::OptionText),
            Self::TAG_FILEXT => Ok(AttributeValueType::Text),
            Self::TAG_NAME => Ok(AttributeValueType::Text),
            Self::TAG_CREATED => Ok(AttributeValueType::DateRange),
            Self::TAG_UPDATED => Ok(AttributeValueType::DateRange),
            _ => Err("Unknown System Tag".to_string()),
        }
    }

    /// from `String` to `SystemTag` \
    /// Also carry the `AttributeValue`
    /// #### example
    /// ```
    /// let tag1 = SystemTag::from_str("@maku:url", AttributeValue::OptionText(Some("Hi")))
    /// let tag2 = SystemTag::from_str("@maku:url", AttributeValue::NumberRange(Some(5), Some(12)))
    /// 
    /// assert_eq!(tag1, SystemTag::URL(Some("Hi")))
    /// assert_eq!(tag1, SystemTag::URL(None))
    /// ```
    pub fn from_str<S: Into<String>>(s: S, value: AttributeValue) -> Result<Self, String> {
        Ok(match s.into().to_lowercase().as_str() {
            Self::TAG_URL => SystemTag::URL(match value {
                AttributeValue::OptionText(text) => text,
                _ => None,
            }),
            Self::TAG_TAGNUM => SystemTag::TagNums(match value {
                AttributeValue::NumberRange(start, end) => (start, end),
                _ => (None, None),
            }),
            Self::TAG_FILE => SystemTag::File(match value {
                AttributeValue::OptionText(text) => text,
                _ => None,
            }),
            Self::TAG_FILEXT => SystemTag::FileExt(match value {
                AttributeValue::Text(text) => text,
                _ => "".to_string(),
            }),
            Self::TAG_NAME => SystemTag::Name(match value {
                AttributeValue::Text(text) => text,
                _ => "".to_string(),
            }),
            Self::TAG_CREATED => SystemTag::CreatedAt(match value {
                AttributeValue::DateRange(start, end) => (start, end),
                _ => (None, None),
            }),
            Self::TAG_UPDATED => SystemTag::UpdatedAt(match value {
                AttributeValue::DateRange(start, end) => (start, end),
                _ => (None, None),
            }),
            _ => Err(String::from("Not Match Functional Tag"))?
        })
    }

    pub fn to_qlstring(&self) -> String {
        match self {
            SystemTag::URL(text) => {
                match text {
                    Some(text) => sql_utils::sql_contain_string("url.full", text),
                    None => "(url)".to_string(),
                }
            },
            SystemTag::File(text) => {
                match text {
                    Some(text) => sql_utils::sql_contain_string("file.name", text),
                    None => "(file)".to_string(),
                }
            },
            SystemTag::Name(text) => {
                sql_utils::sql_contain_string("fname", text)
            },
            SystemTag::FileExt(text) => {
                sql_utils::sql_contain_string("file.ext", text)
            },
            SystemTag::TagNums((start, end)) => {
                sql_utils::sql_range_number("count(<-tagging<-tag.id)", (start, end))
            },
            SystemTag::CreatedAt((date1, date2)) => {
                sql_utils::sql_range_date("created_at", (date1, date2))
            },
            SystemTag::UpdatedAt((date1, date2)) => {
                sql_utils::sql_range_date("updated_at", (date1, date2))
            }
        }
    }
}
