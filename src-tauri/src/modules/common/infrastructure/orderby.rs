use std::str::FromStr;
use regex::Regex;

use crate::utils::StringUtils;

#[derive(Clone)]
pub enum OrderByType {
    ASC,
    DESC,
}
impl ToString for OrderByType {
    fn to_string(&self) -> String {
        match self {
            Self::ASC => "ASC".to_string(),
            Self::DESC => "DESC".to_string(),
        }
    }
}

pub struct OrderBy(String, OrderByType);

impl OrderBy {
    const ORDER_BY_FORMAT: &str = r"[<]?[A-Za-z_]+";
}

impl OrderBy {
    pub fn get_field(&self) -> &String {
        &self.0
    }

    pub fn get_order_type(&self) -> &OrderByType {
        &self.1
    }
}

impl FromStr for OrderBy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(Self::ORDER_BY_FORMAT).unwrap();
        if regex.is_match(s) == false {
            return Err(format!("Invalid Order String format: {}", Self::ORDER_BY_FORMAT));
        }
        Ok(match s.starts_with("<") {
            true => Self(s.slice(1..).to_string(), OrderByType::ASC),
            false => Self(s.to_string(), OrderByType::DESC),
        })
    }
}
