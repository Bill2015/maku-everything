use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::modules::common::infrastructure::dateutils;
use crate::modules::tag::domain::TagGenericError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TagAttrVO {
    Normal,

    Number { start: i64, end: i64, defval: i64 },

    Text { defval: String },

    Date { defval: DateTime<Utc> },

    Bool { defval: bool },
}

pub struct TagAttributeFactory { }

impl TagAttributeFactory {
    pub fn create_normal() -> Result<TagAttrVO, TagGenericError> {
        Ok(TagAttrVO::Normal)
    }

    pub fn create_number(start: i64, end: i64, defval: i64) -> Result<TagAttrVO, TagGenericError> {
        if start > end {
            return Err(TagGenericError::InvalidTagNumberValue());
        }
        if defval < start || defval > end {
            return Err(TagGenericError::InvalidTagNumberValue());
        }

        Ok(TagAttrVO::Number{ start, end, defval })
    }

    pub fn create_text(defval: String) -> Result<TagAttrVO, TagGenericError> {
        Ok(TagAttrVO::Text { defval })
    }

    pub fn create_date(defval: String) -> Result<TagAttrVO, TagGenericError> {
        if let Ok(date) = dateutils::parse(defval) {
            return Ok(TagAttrVO::Date { defval: date.and_utc() })
        }

        Err(TagGenericError::InvalidDateFormat())
    }

    pub fn create_bool(defval: bool) -> Result<TagAttrVO, TagGenericError> {
        Ok(TagAttrVO::Bool { defval })
    }
}
