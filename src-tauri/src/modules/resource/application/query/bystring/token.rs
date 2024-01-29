use crate::resource::infrastructure::AttributeValue;

use super::types::TokenSymbol;

#[derive(Debug, Clone)]
pub enum QueryToken {
    SymbolToken{
        symbol: TokenSymbol,
        value: String,
    },
    TagToken{
        symbol: TokenSymbol,
        id: String,
        namespace: Option<String>,
        value: String,
    },
    SystemTagToken{
        namespace: String,
        symbol: TokenSymbol,
        value: String,
        attrval: Option<AttributeValue>,
    },
    AttributeToken{
        symbol: TokenSymbol,
        value: String,
    },
}

impl QueryToken {
    pub fn new_symbol(symbol: TokenSymbol, value: String) -> Self {
        Self::SymbolToken{ symbol, value }
    }

    pub fn new_tag(symbol: TokenSymbol, namespace: Option<String>, value: String) -> Self {
        Self::TagToken{
            id: String::default(),
            symbol,
            value,
            namespace,
        }
    }

    pub fn new_system_tag(symbol: TokenSymbol, namespace: String, value: String) -> Self {
        Self::SystemTagToken { symbol, namespace, value, attrval: None }
    }

    pub fn new_attribute(symbol: TokenSymbol, value: String) -> Self {
        Self::AttributeToken {
            symbol,
            value,
        }
    }

    pub fn set_attribute(&mut self, val: AttributeValue) {
        if let Self::SystemTagToken { attrval, .. } = self {
            *attrval = Some(val);
        }
    }

    pub fn set_tag_id(&mut self, val: String) {
        if let Self::TagToken { id, .. } = self {
            *id = val;
        }
    }
}
