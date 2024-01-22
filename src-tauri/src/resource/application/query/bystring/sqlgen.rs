use crate::resource::domain::ResourceError;
use crate::resource::infrastructure::{StringQLObject, StringQLObjectBuilder, StringQLPrefix, StringQLTagItem, SystemTag};

use super::types::TokenSymbol;
use super::token::QueryToken;

struct Symbol<'a>(&'a TokenSymbol);

pub struct SQLQueryObjectGenerator<'a> {
    tokens: &'a Vec<QueryToken>,

    builder: StringQLObjectBuilder,
}

impl<'a> SQLQueryObjectGenerator<'a> {
    pub fn new(tokens: &'a Vec<QueryToken>) -> Self {
        Self { 
            tokens, 
            builder: StringQLObjectBuilder::new(),
        }
    }

    pub fn gen(&mut self) -> Result<StringQLObject, ResourceError> {

        let mut ops_stack: Vec<Symbol> = Vec::new();

        let mut tag_stack: Vec<StringQLTagItem> = Vec::new();

        for token in self.tokens {
            match token {
                QueryToken::SymbolToken{ symbol, value: _ } => {
                    // if match the 'Right Group Bracket', start popup tags
                    if *symbol == TokenSymbol::RightGroupBracket {
                        let mut tags: Vec<StringQLTagItem> = Vec::new();

                        while !tag_stack.is_empty() {
                            if let Some(item) = tag_stack.pop() {
                                tags.push(item);
                            }
                            else {
                                break;
                            }
                        }
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
                QueryToken::AttributeToken { .. } => { },
                QueryToken::TagToken { id, .. } => {
                    match ops_stack.last().unwrap().0 {
                        TokenSymbol::Include => {
                            self.builder.add_item(StringQLTagItem::new(StringQLPrefix::Include, id.to_string(), None));
                        },
                        TokenSymbol::Exclude => {
                            self.builder.add_item(StringQLTagItem::new(StringQLPrefix::Exclude, id.to_string(), None));
                        }
                        _ => {
                            tag_stack.push(StringQLTagItem::new(StringQLPrefix::Inherit, id.to_string(), None));
                        },
                    }
                },
                QueryToken::SystemTagToken { namespace, value, attrval, .. } => {
                    let name = SystemTag::full_name(namespace, value);
                    match ops_stack.last().unwrap().0 {
                        TokenSymbol::Include => {
                            self.builder.add_item(StringQLTagItem::new(StringQLPrefix::Include, name, attrval.clone()));
                        },
                        TokenSymbol::Exclude => {
                            self.builder.add_item(StringQLTagItem::new(StringQLPrefix::Exclude, name, attrval.clone()));
                        }
                        _ => {
                            tag_stack.push(StringQLTagItem::new(StringQLPrefix::Inherit, name, attrval.clone()));
                        },
                    }
                }
            }
        };

        Ok(self.builder.build())
    }
}