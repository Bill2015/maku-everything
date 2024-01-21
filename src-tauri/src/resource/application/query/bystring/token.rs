use super::types::TokenSymbol;

#[derive(Debug, Clone)]
pub enum QueryToken {
    SymbolToken{ symbol: TokenSymbol, value: String },
    TagToken{ symbol: TokenSymbol, namespace: Option<String>, value: String },
    AttributeToken{ symbol: TokenSymbol, value: String },
}

impl QueryToken {
    pub fn new_symbol(symbol: TokenSymbol, value: String) -> Self {
        Self::SymbolToken{ symbol, value }
    }

    pub fn new_tag(symbol: TokenSymbol, namespace: Option<String>, value: String) -> Self {
        Self::TagToken{
            symbol,
            value,
            namespace,
        }
    }

    pub fn new_attribute(symbol: TokenSymbol, value: String) -> Self {
        Self::AttributeToken {
            symbol,
            value,
        }
    }
}
