use std::num::{IntErrorKind, ParseIntError};

use chrono::NaiveDate;

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

pub enum AttributeParseError {
    MultipleRangeFound,
    InvlidRangeNumberFormat,
    InvlidRangeDateFormat,
    RangeEmpty,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {
    Text(String),
    OptionText(Option<String>),
    NumberRange(Option<usize>, Option<usize>),
    DateRange(Option<String>, Option<String>),
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
            return Err(AttributeParseError::InvlidRangeNumberFormat);
        }

        let nums: Vec<_> = nums.into_iter().map(|x| x.unwrap()).collect();
        if nums.iter().all(Option::is_none) {
            return Err(AttributeParseError::RangeEmpty);
        }

        Ok(Self::NumberRange(nums[0], nums[1]))
    }

    fn parse_date_range(value: &str) -> Result<Self, AttributeParseError> {
        // single date
        if let Ok(_) = NaiveDate::parse_from_str(value, "%Y/%m/%d") {
            return Ok(Self::DateRange(Some(value.to_string()), Some(value.to_string())));
        }

        let date = Self::get_range(value)?
            .iter()
            .map(|val| {
                if let Ok(_) = NaiveDate::parse_from_str(val, "%Y/%m/%d") {
                    return Ok(Some(val.to_string()))
                }
                if val.is_empty() {
                    return Ok(None)
                }
                Err(())
            })
            .collect::<Vec<Result<Option<String>, ()>>>();
    
        if !date.iter().all(Result::is_ok) {
            return Err(AttributeParseError::InvlidRangeDateFormat);
        }

        let date: Vec<_> = date.into_iter().map(|x| x.unwrap()).collect();
        if date.iter().all(Option::is_none) {
            return Err(AttributeParseError::RangeEmpty);
        }

        Ok(Self::DateRange(date[0].to_owned(), date[1].to_owned()))
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
    pub fn parse_from(val: &str, attr_type: AttributeValueType) -> Result<Self, AttributeParseError> {
        match attr_type {
            AttributeValueType::Text => {
                Ok(Self::Text(val.to_string()))
            },
            AttributeValueType::OptionText => {
                Ok(Self::OptionText(
                    match !val.is_empty() {
                        true => Some(val.to_string()),
                        false => None,
                    }
                ))
            },
            AttributeValueType::NumberRange => {
                let result = Self::parse_number_range(val)?;
                Ok(result)
            },
            AttributeValueType::DateRange => {
                let result = Self::parse_date_range(val)?;
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
pub struct StringQLTagItem {
    prefix: StringQLPrefix,

    value: String,

    attribute: Option<AttributeValue>,
}
impl StringQLTagItem {
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

    pub items: Vec<StringQLTagItem>,
}

// ---------------------------------------------------------
/// Immutable String QL Object \
/// Using `StringQLObjectBuilder` to build object
#[derive(Debug, Clone)]
pub struct StringQLObject {
    item: Vec<StringQLTagItem>,

    groups: Vec<StringQLGroup>,
}

impl StringQLObject {
    pub fn new() -> Self {
        Self { item: Vec::new(), groups: Vec::new() }
    }

    pub fn get_items(&self) -> &Vec<StringQLTagItem> {
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

    pub fn add_group(&mut self, prefix: StringQLPrefix, items: Vec<StringQLTagItem>) {
        self.obj.groups.push(StringQLGroup { prefix, items });
    }

    pub fn add_item(&mut self, item: StringQLTagItem) {
        self.obj.item.push(item);
    }

    pub fn build(&self) -> StringQLObject {
        self.obj.to_owned()
    }
}