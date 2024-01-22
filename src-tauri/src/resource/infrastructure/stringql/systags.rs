use crate::common::repository::sql_predefn;

use super::{AttributeValue, AttributeValueType};

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
    pub fn full_name(namespace: &str, val: &str) -> String {
        format!("{}:{}", namespace, val)
    }

    pub fn is_defined(val: &str) -> bool {
        match val.to_lowercase().as_str() {
            "@maku:url" => true,
            "@maku:tagnum" => true,
            "@maku:file" => true,
            "@maku:filext" => true,
            "@maku:name" => true,
            "@maku:created" => true,
            "@maku:updated" => true,
            _ => false,
        }
    }

    pub fn attr_type(namespace: &String, name: &String) -> Result<AttributeValueType, String> {
        let s = format!("{}:{}", namespace, name);
        match s.to_lowercase().as_str() {
            "@maku:url" => Ok(AttributeValueType::OptionText),
            "@maku:tagnum" => Ok(AttributeValueType::NumberRange),
            "@maku:file" => Ok(AttributeValueType::OptionText),
            "@maku:filext" => Ok(AttributeValueType::Text),
            "@maku:name" => Ok(AttributeValueType::Text),
            "@maku:created" => Ok(AttributeValueType::DateRange),
            "@maku:updated" => Ok(AttributeValueType::DateRange),
            _ => Err("Unknown System Tag".to_string()),
        }
    }

    pub fn from_str(s: &str, value: AttributeValue) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "@maku:url" => Ok(SystemTag::URL(match value {
                AttributeValue::OptionText(text) => text,
                _ => None,
            })),
            "@maku:tagnum" => Ok(SystemTag::TagNums(match value {
                AttributeValue::NumberRange(start, end) => (start, end),
                _ => (None, None),
            })),
            "@maku:file" => Ok(SystemTag::File(match value {
                AttributeValue::OptionText(text) => text,
                _ => None,
            })),
            "@maku:filext" => Ok(SystemTag::FileExt(match value {
                AttributeValue::Text(text) => text,
                _ => "".to_string(),
            })),
            "@maku:name" => Ok(SystemTag::Name(match value {
                AttributeValue::Text(text) => text,
                _ => "".to_string(),
            })),
            "@maku:created" => Ok(SystemTag::CreatedAt(match value {
                AttributeValue::DateRange(start, end) => (start, end),
                _ => (None, None),
            })),
            "@maku:updated" => Ok(SystemTag::UpdatedAt(match value {
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
