use crate::common::repository::sql_predefn;

use super::{AttributeValue, AttributeValueType};


///
/// System Pre-defined Tag
pub enum SystemTag {
    URL(Option<String>),
    TagNums((Option<usize>, Option<usize>)),
    File(Option<String>),
    FileExt(String),
    Name(String),
    CreatedAt((Option<String>, Option<String>)),
    UpdatedAt((Option<String>, Option<String>)),
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
        match s.into().to_lowercase().as_str() {
            Self::TAG_URL => Ok(SystemTag::URL(match value {
                AttributeValue::OptionText(text) => text,
                _ => None,
            })),
            Self::TAG_TAGNUM => Ok(SystemTag::TagNums(match value {
                AttributeValue::NumberRange(start, end) => (start, end),
                _ => (None, None),
            })),
            Self::TAG_FILE => Ok(SystemTag::File(match value {
                AttributeValue::OptionText(text) => text,
                _ => None,
            })),
            Self::TAG_FILEXT => Ok(SystemTag::FileExt(match value {
                AttributeValue::Text(text) => text,
                _ => "".to_string(),
            })),
            Self::TAG_NAME => Ok(SystemTag::Name(match value {
                AttributeValue::Text(text) => text,
                _ => "".to_string(),
            })),
            Self::TAG_CREATED => Ok(SystemTag::CreatedAt(match value {
                AttributeValue::DateRange(start, end) => (start, end),
                _ => (None, None),
            })),
            Self::TAG_UPDATED => Ok(SystemTag::UpdatedAt(match value {
                AttributeValue::DateRange(start, end) => (start, end),
                _ => (None, None),
            })),
            _ => Err(String::from("Not Match Functional Tag"))
        }
    }

    pub fn to_qlstring(&self, not_flag: bool) -> String {
        let content = match self {
            SystemTag::URL(text) => {
                match text {
                    Some(text) => format!("(url.full CONTAINS \"{}\")", text),
                    None => "(url)".to_string(),
                }
            },
            SystemTag::File(text) => {
                match text {
                    Some(text) => format!("(file.name CONTAINS \"{}\")", text),
                    None => "(file)".to_string(),
                }
            },
            SystemTag::Name(text) => {
                format!("(name CONTAINS \"{}\")", text)
            },
            SystemTag::FileExt(text) => {
                format!("(file.ext CONTAINS \"{}\")", text)
            },
            SystemTag::TagNums((start, end)) => {
                if start.is_some() && end.is_some() {
                    return sql_predefn::between("count(<-tagging<-tag.id)", start.unwrap(), end.unwrap())
                }
                if start.is_some() {
                    return format!("(count(<-tagging<-tag.id) >= {})", start.unwrap())
                }
                else {
                    return format!("(count(<-tagging<-tag.id) <= {})", end.unwrap())
                }
            },
            // TODO:
            _ => { "".to_string() }
        };

        let not_symbol = if not_flag { "!" } else { "" };
        format!("{}{}", not_symbol, content)
    }
}
