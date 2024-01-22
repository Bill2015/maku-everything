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

impl TokenSymbol {
    pub fn from_str<S: Into<String>>(s: S) -> Result<Self, String> {
        match s.into().to_lowercase().as_str() {
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
pub enum QueryingStrChar {
    StringWrapper,
    NamespaceDelimiter,
}


impl PartialEq<char> for QueryingStrChar {
    fn eq(&self, other: &char) -> bool {
        match self {
            Self::NamespaceDelimiter => ':' == *other,
            Self::StringWrapper => '"' == *other,
        }
    }
}
