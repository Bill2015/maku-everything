use std::num::IntErrorKind;

use chrono::NaiveDate;

// ---------------------------------------------------------
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
    fn parse_number_range(val: &str) -> Result<Self, AttributeParseError> {
                
        // single number
        if let Ok(num) = val.parse::<usize>() {
            return Ok(Self::NumberRange(Some(num), Some(num)));
        }

        let (start, end) = Self::get_range(val)?;
        let string_nums: Vec<&str> = vec![start, end];
        let nums: Vec<_> = string_nums.iter()
            .map(|val| {
                val.parse::<i64>()
                    .or_else(|x| match x.kind() {
                        IntErrorKind::Empty => Ok(-1),
                        _ => Err(x),
                    })
            })
            .collect();
        
        if nums[0].is_ok() && nums[1].is_ok() {
            let start_num = nums[0].clone().unwrap();
            let end_num = nums[1].clone().unwrap();

            return Ok(
                Self::NumberRange(
                    if start_num < 0 { None } else { Some(start_num as usize) },
                    if end_num < 0 { None } else { Some(end_num as usize) },
                )
            )
        }

        Err(AttributeParseError::InvlidRangeNumberFormat)
    }

    fn parse_date_range(val: &str) -> Result<Self, AttributeParseError> {
        // single date
        if let Ok(_) = NaiveDate::parse_from_str(val, "%Y/%m/%d") {
            return Ok(Self::DateRange(Some(val.to_string()), Some(val.to_string())));
        }

        let (start, end) = Self::get_range(val)?;
        let string_date: Vec<&str> = vec![start, end];
        let date: Vec<_> = string_date.iter()
            .map(|val| {
                if let Ok(_) = NaiveDate::parse_from_str(val, "%Y/%m/%d") {
                    return Ok(val)
                }
                if val.is_empty() {
                    return Ok(val)
                }
                return Err(());
            })
            .collect();

        if date[0].is_ok() && date[1].is_ok() {
            let start_date = date[0].unwrap();
            let end_date = date[1].unwrap();

            return Ok(
                Self::DateRange(
                    if start_date.is_empty() { None } else { Some(start_date.to_string()) },
                    if end_date.is_empty() { None } else { Some(end_date.to_string()) },
                )
            )
        }

        Err(AttributeParseError::InvlidRangeDateFormat)
    }

    fn get_range(val: &str) -> Result<(&str, &str), AttributeParseError> {
        let separated: Vec<&str> = val.split("..").collect();
        if separated.len() == 2 {
            return Ok((
                separated.get(0).unwrap(), 
                separated.get(1).unwrap(),
            ));
        }
        if separated.len() > 2 {
            return Err(AttributeParseError::MultipleRangeFound)
        }
        Err(AttributeParseError::MultipleRangeFound)
    }

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

    excludes: Vec<StringQLTagItem>,

    includes: Vec<StringQLTagItem>,

    groups: Vec<StringQLGroup>,
}

impl StringQLObject {
    pub fn new() -> Self {
        Self { item: Vec::new(), excludes: Vec::new(), includes: Vec::new(), groups: Vec::new() }
    }

    pub fn get_excludes(&self) -> &Vec<StringQLTagItem> {
        &self.excludes
    }

    pub fn get_includes(&self) -> &Vec<StringQLTagItem>{
        &self.includes
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