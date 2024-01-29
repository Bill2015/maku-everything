use std::fmt::Debug;
use std::num::{IntErrorKind, ParseIntError};

use chrono::NaiveDate;

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

    #[error("Range is empty")]
    RangeEmpty,
}

// ---------------------------------------------------------
/// Defined type of attribute
/// ```
/// Text => String
/// OptionText => Option<String>
/// NumberRange => (Option<usize>, Option<usize>)
/// DateRange => (Option<String>, Option<String>)
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValueType {
    Text,
    OptionText,
    NumberRange,
    DateRange,
}

// ---------------------------------------------------------
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {
    Text(String),
    OptionText(Option<String>),
    NumberRange(Option<usize>, Option<usize>),
    DateRange(Option<NaiveDate>, Option<NaiveDate>),
    None,
}
impl AttributeValue {

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
        }
    }
}

// ---------------------------------------------------------
#[derive(Debug, Clone)]
pub enum StringQLPrefix {
    Include,
    Exclude,
    Inherit,
}

// ---------------------------------------------------------
#[derive(Debug, Clone)]
pub struct StringQLItem {
    prefix: StringQLPrefix,

    value: String,

    attribute: Option<AttributeValue>,
}
impl StringQLItem {
    pub fn new( prefix: StringQLPrefix, value: String, attribute: Option<AttributeValue>) -> Self {
        Self { prefix, value, attribute }
    }

    pub fn get_attribute(&self) -> &Option<AttributeValue> {
        &self.attribute
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }

    pub fn get_prefix(&self) -> &StringQLPrefix {
        &self.prefix
    }
}

#[derive(Debug, Clone)]
pub struct StringQLGroup {
    pub prefix: StringQLPrefix,

    pub items: Vec<StringQLItem>,
}

// ---------------------------------------------------------
/// Immutable String QL Object \
/// Using `StringQLObjectBuilder` to build object
#[derive(Debug, Clone)]
pub struct StringQLObject {
    belong_category: Option<String>,

    item: Vec<StringQLItem>,

    groups: Vec<StringQLGroup>,
}

impl StringQLObject {
    pub fn new() -> Self {
        Self { item: Vec::new(), groups: Vec::new(), belong_category: None }
    }

    pub fn get_items(&self) -> &Vec<StringQLItem> {
        &self.item
    }

    pub fn get_groups(&self) ->  &Vec<StringQLGroup> {
        &self.groups
    }
    
}

// ---------------------------------------------------------
pub struct StringQLObjectBuilder {
    obj: StringQLObject,
}

impl StringQLObjectBuilder {
    pub fn new() -> Self {
        Self { obj: StringQLObject::new() }
    }

    pub fn set_belong_category(&mut self, belong_category: String) {
        self.obj.belong_category = Some(belong_category);
    }

    pub fn add_group(&mut self, prefix: StringQLPrefix, items: Vec<StringQLItem>) {
        self.obj.groups.push(StringQLGroup { prefix, items });
    }

    pub fn add_item(&mut self, item: StringQLItem) {
        self.obj.item.push(item);
    }

    pub fn build(&self) -> StringQLObject {
        self.obj.to_owned()
    }
}