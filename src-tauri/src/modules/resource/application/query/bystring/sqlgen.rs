use crate::modules::resource::domain::ResourceGenericError;
use crate::modules::resource::infrastructure::{StringQLObject, StringQLObjectBuilder, StringQLPrefix, StringQLItem, SystemTag};

use super::types::TokenSymbol;
use super::token::QueryToken;

struct Symbol<'a>(&'a TokenSymbol);

pub struct StringQLObjectGenerator<'a> {
    tokens: &'a Vec<QueryToken>,

    builder: StringQLObjectBuilder,
}

impl<'a> StringQLObjectGenerator<'a> {
    pub fn new(tokens: &'a Vec<QueryToken>) -> Self {
        Self { 
            tokens, 
            builder: StringQLObjectBuilder::new(),
        }
    }

    pub fn set_belong_category(mut self, belong_category: Option<String>) -> Self {
        if let Some(category) = belong_category {
            self.builder.set_belong_category(category);
        }
        self
    }

    pub fn gen(&mut self) -> Result<StringQLObject, ResourceGenericError> {

        let mut ops_stack: Vec<Symbol> = Vec::new();

        let mut tag_stack: Vec<StringQLItem> = Vec::new();

        for token in self.tokens {
            match token {
                QueryToken::SymbolToken{ symbol, value: _ } => {
                    // if match the 'Right Group Bracket', start popup tags
                    if *symbol == TokenSymbol::RightGroupBracket {
                        let tags: Vec<StringQLItem> = tag_stack.drain(..).collect();

                        // pop the 'LeftGroupBracket'
                        ops_stack.pop();
                        match ops_stack.last().unwrap().0 {
                            TokenSymbol::Include => {
                                self.builder.add_group(StringQLPrefix::Include, tags);
                                ops_stack.pop();
                            },
                            TokenSymbol::Exclude => {
                                self.builder.add_group(StringQLPrefix::Exclude, tags);
                                ops_stack.pop();
                            },
                            _ => {},
                        }
                    }

                    match symbol {
                        TokenSymbol::Include => ops_stack.push(Symbol(symbol)),
                        TokenSymbol::Exclude => ops_stack.push(Symbol(symbol)),
                        TokenSymbol::LeftGroupBracket => ops_stack.push(Symbol(symbol)),
                        _ => {}
                    };
                },
                QueryToken::TagToken { id, attrval, .. } => {
                    let (id, attrval) = (id.to_string(), attrval.clone());
                    match ops_stack.last().unwrap().0 {
                        TokenSymbol::Include => {
                            self.builder.add_item(StringQLItem::new(StringQLPrefix::Include, id, attrval, false));
                        },
                        TokenSymbol::Exclude => {
                            self.builder.add_item(StringQLItem::new(StringQLPrefix::Exclude, id, attrval, false));
                        }
                        _ => {
                            tag_stack.push(StringQLItem::new(StringQLPrefix::Inherit, id, attrval, false));
                        },
                    }
                },
                QueryToken::SystemTagToken { namespace, value, attrval, .. } => {
                    let attrval = attrval.clone();
                    let name = SystemTag::full_name(namespace, value);
                    match ops_stack.last().unwrap().0 {
                        TokenSymbol::Include => {
                            self.builder.add_item(StringQLItem::new(StringQLPrefix::Include, name, attrval, true));
                        },
                        TokenSymbol::Exclude => {
                            self.builder.add_item(StringQLItem::new(StringQLPrefix::Exclude, name, attrval, true));
                        }
                        _ => {
                            tag_stack.push(StringQLItem::new(StringQLPrefix::Inherit, name, attrval, true));
                        },
                    }
                },
                // semantic will merge all the attribute tokens in tag token
                QueryToken::AttributeToken { .. } => {
                    return Err(ResourceGenericError::InvalidQueryingString{ 
                        message: "Unexpected error, it still have attribute token in string ql generator".to_string()
                    })
                },
            }
        };

        Ok(self.builder.build())
    }
}