use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct QueryToken {
    pub symbol: TokenSymbol,
    /// to defined the namespace of value
    pub namespace: Option<String>,

    pub value: String,
}

impl QueryToken {
    pub fn new(symbol: TokenSymbol, namespace: Option<String>, value: String) -> Self {
        Self { symbol, value, namespace }
    }
}

/// For Token Definition
#[derive(Debug, Clone, PartialEq)]
pub enum TokenSymbol {
    TagName,
    Include,
    Exclude,
    EOF,
    LeftBracket,
    RightBracket,
}

impl FromStr for TokenSymbol {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "+" => Ok(Self::Include),
            "-" => Ok(Self::Exclude),
            "[" => Ok(Self::LeftBracket),
            "]" => Ok(Self::RightBracket),
            "$" => Ok(Self::EOF),
            _ => Err(String::from("Not Match Operator Symbol")) 
        }
    }
}

pub enum QueryingStringSymbol {
    TagNameWrapper,
    SubjectDelimiter,
}


impl PartialEq<char> for QueryingStringSymbol {
    fn eq(&self, other: &char) -> bool {
        match self {
            Self::SubjectDelimiter => ':' == *other,
            Self::TagNameWrapper => '\"' == *other,
        }
    }
}

impl FromStr for QueryingStringSymbol {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "\"" => Ok(Self::TagNameWrapper),
            ":" => Ok(Self::SubjectDelimiter),
            _ =>  Err(String::from("Not Match Querying String Symbol")) 
        }
    }
}

impl ToString for QueryingStringSymbol {
    fn to_string(&self) -> String {
        match self {
            Self::SubjectDelimiter => "\"".to_string(),
            Self::TagNameWrapper => ":".to_string(),
        }
    }
}
