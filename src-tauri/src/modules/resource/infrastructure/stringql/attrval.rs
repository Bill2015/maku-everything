use std::fmt::Debug;
use std::num::{IntErrorKind, ParseIntError};

use chrono::NaiveDate;
use strum_macros::EnumDiscriminants;

#[derive(thiserror::Error, Debug)]
pub enum AttributeParseError {
    #[error("Multiple range found")]
    MultipleRangeFound,

    #[error("Invalid range number format")]
    InvalidRangeNumberFormat,

    #[error("Range numeber start is larger than the end")]
    NumberStartIsLargerThanEnd,

    #[error("Invalid range date format")]
    InvalidRangeDateFormat,
    
    #[error("Range date start is larger than the end")]
    DateStartIsLargerThenEnd,

    #[error("Invalid bool format")]
    InvalidBoolFormat,

    #[error("Range is empty")]
    RangeEmpty,
}


// ---------------------------------------------------------
#[derive(Debug, Clone, PartialEq, EnumDiscriminants)]
#[strum_discriminants(name(AttributeValueType))]
pub enum AttributeValue {
    Text(String),
    OptionText(Option<String>),
    NumberRange(Option<usize>, Option<usize>),
    DateRange(Option<NaiveDate>, Option<NaiveDate>),
    Bool(bool),
    None,
}
impl AttributeValue {

    fn parse_bool(value: &str) -> Result<Self, AttributeParseError> {
        Ok(match value {
            "" => Self::Bool(true),
            "true" | "1" => Self::Bool(true),
            "false" | "0" => Self::Bool(false),
            _ => Err(AttributeParseError::InvalidBoolFormat)?,
        })
    }

    fn parse_number_range(value: &str) -> Result<Self, AttributeParseError> {
        // single number
        if let Ok(num) = value.parse::<usize>() {
            return Ok(Self::NumberRange(Some(num), Some(num)));
        }

        let nums = Self::get_range(value)?
            .iter()
            .map(|val| {
                val.parse::<i64>()
                    .or_else(|x| match x.kind() { IntErrorKind::Empty => Ok(-1), _ => Err(x) })
                    .and_then(|x| match x {
                        x if x >= 0 => Ok(Some(x as usize)),
                        _ => Ok(None),
                    })
            })
            .collect::<Vec<Result<Option<usize>, ParseIntError>>>();

        if !nums.iter().all(Result::is_ok) {
            return Err(AttributeParseError::InvalidRangeNumberFormat);
        }

        let nums: Vec<_> = nums.into_iter().map(|x| x.unwrap()).collect();
        if nums.iter().all(Option::is_none) {
            return Err(AttributeParseError::RangeEmpty);
        }

        if let (Some(start), Some(end)) = (nums[0], nums[1]) {
            if start > end {
                return Err(AttributeParseError::NumberStartIsLargerThanEnd);
            }
        }

        Ok(Self::NumberRange(nums[0], nums[1]))
    }

    fn parse_date_range(value: &str) -> Result<Self, AttributeParseError> {
        // single date
        if let Ok(val) = NaiveDate::parse_from_str(value, "%Y/%m/%d") {
            return Ok(Self::DateRange(Some(val), Some(val)));
        }

        let date = Self::get_range(value)?
            .iter()
            .map(|val| {
                if let Ok(date) = NaiveDate::parse_from_str(val, "%Y/%m/%d") {
                    return Ok(Some(date))
                }
                if val.is_empty() {
                    return Ok(None)
                }
                Err(())
            })
            .collect::<Vec<Result<Option<NaiveDate>, ()>>>();
    
        if !date.iter().all(Result::is_ok) {
            return Err(AttributeParseError::InvalidRangeDateFormat);
        }

        let date: Vec<_> = date.into_iter().map(|x| x.unwrap()).collect();
        if date.iter().all(Option::is_none) {
            return Err(AttributeParseError::RangeEmpty);
        }

        if let (Some(start), Some(end)) = (date[0], date[1]) {
            if start > end {
                return Err(AttributeParseError::DateStartIsLargerThenEnd);
            }
        }

        Ok(Self::DateRange(date[0], date[1].to_owned()))
    }

    /// separate the range from string
    /// 
    /// ### example:
    /// ```
    /// let range1 = get_range("1..45");
    /// let range2 = get_range("1..");
    /// let range3 = get_range("1..15..40");
    /// 
    /// assert_eq!(range1, Ok(vect!["1", "45"])); 
    /// assert_eq!(range1, Ok(vect!["1", ""])); 
    /// assert_eq!(range1, Err(AttributeParseError::MultipleRangeFound)); 
    /// ```
    fn get_range(val: &str) -> Result<Vec<&str>, AttributeParseError> {
        let separated: Vec<&str> = val.split("..").collect();
        if separated.len() == 2 {
            return Ok(vec![separated.get(0).unwrap(), separated.get(1).unwrap()]);
        }
        if separated.len() > 2 {
            return Err(AttributeParseError::MultipleRangeFound)
        }
        Err(AttributeParseError::MultipleRangeFound)
    }

    /// depend on `AttributeValueType` and converte val to `AttributeValue`
    /// ### example:
    /// ```
    /// let a = parse_from("45", AttributeValueType.Text);
    /// let b = parse_from("..45", AttributeValueType.NumberRange);
    /// let c = parse_from("2021/06/23..", AttributeValueType.DateRange);
    /// 
    /// assert_eq!(a, Ok(AttributeValue::Text("45".to_string())));
    /// assert_eq!(b, Ok(AttributeValue::NumberRange(None, Some(45))));
    /// assert_eq!(c, Ok(AttributeValue::DateRange(Some("2021/06/23..".to_string()), None)));
    /// ```
    pub fn parse_from<T: Into<String>>(s: T, attr_type: AttributeValueType) -> Result<Self, AttributeParseError> {
        let val = s.into();
        match attr_type {
            AttributeValueType::Text => {
                Ok(Self::Text(val))
            },
            AttributeValueType::OptionText => {
                Ok(Self::OptionText(
                    match !val.is_empty() {
                        true => Some(val),
                        false => None,
                    }
                ))
            },
            AttributeValueType::NumberRange => {
                let result = Self::parse_number_range(&val)?;
                Ok(result)
            },
            AttributeValueType::DateRange => {
                let result = Self::parse_date_range(&val)?;
                Ok(result)
            },
            AttributeValueType::Bool => {
                let result = Self::parse_bool(&val)?;
                Ok(result)
            },
            AttributeValueType::None => {
                Ok(Self::None)
            }
        }
    }
}
