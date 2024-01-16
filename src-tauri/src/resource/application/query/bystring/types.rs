use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct QueryToken {
    pub value: String,
    pub token_name: TokenSymbol,
}

impl QueryToken {
    pub fn new(value: String, token_name: TokenSymbol) -> Self {
        Self { value, token_name }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenSymbol {
    TagName,
    Include,
    Exclude,
    EOF,
    LeftBracket,
    RightBracket,
}

impl TokenSymbol {
    pub fn as_str(&self) -> &str {
        match self {
            Self::TagName => "Tag Name",
            Self::Include => "+",
            Self::Exclude => "-",
            Self::LeftBracket => "[",
            Self::RightBracket => "]",
            Self::EOF => "$",
        }
    }
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

