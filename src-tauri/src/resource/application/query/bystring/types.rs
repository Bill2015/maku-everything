use std::str::FromStr;

// ---------------------------------------------------------
/// For Token Definition
#[derive(Debug, Clone, PartialEq)]
pub enum TokenSymbol {
    TagName,
    Attribute,
    Include,
    Exclude,
    EOF,
    LeftGroupBracket,
    RightGroupBracket,
    LeftAttrBracket,
    RightAttrBracket,
}

impl FromStr for TokenSymbol {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "+" => Ok(Self::Include),
            "-" => Ok(Self::Exclude),
            "[" => Ok(Self::LeftGroupBracket),
            "]" => Ok(Self::RightGroupBracket),
            "{" => Ok(Self::LeftAttrBracket),
            "}" => Ok(Self::RightAttrBracket),
            "$" => Ok(Self::EOF),
            _ => Err(String::from("Not Match Operator Symbol")) 
        }
    }
}

impl PartialEq<char> for TokenSymbol {
    fn eq(&self, other: &char) -> bool {
        match self {
            Self::TagName => false,
            Self::Attribute => false,
            Self::Include => '+' == *other,
            Self::Exclude => '\"' == *other,
            Self::LeftGroupBracket => '[' == *other,
            Self::RightGroupBracket => ']' == *other,
            Self::LeftAttrBracket => '{' == *other,
            Self::RightAttrBracket => '{' == *other,
            Self::EOF => '$' == *other,
        }
    }
}

// ---------------------------------------------------------
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
