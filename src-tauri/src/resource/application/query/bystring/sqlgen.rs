use std::collections::HashMap;

use crate::resource::domain::ResourceError;
use crate::resource::infrastructure::{StringQLGroupPrefix, StringQLObject, StringQLObjectBuilder, StringQLTagItem};

use super::types::TokenSymbol;
use super::token::QueryToken;

struct Symbol<'a>(&'a TokenSymbol);

pub struct SQLQueryObjectGenerator<'a> {
    tokens: &'a Vec<QueryToken>,
    /** For tag id mapping  */
    tag_id_map: HashMap<String, String>,

    builder: StringQLObjectBuilder,
}

impl<'a> SQLQueryObjectGenerator<'a> {
    pub fn new(tokens: &'a Vec<QueryToken>, tag_id_map: HashMap<String, String>) -> Self {
        Self { 
            tokens, 
            tag_id_map,
            builder: StringQLObjectBuilder::new(),
        }
    }

    pub fn gen(&mut self) -> Result<StringQLObject, ResourceError> {

        let mut ops_stack: Vec<Symbol> = Vec::new();

        let mut tag_stack: Vec<StringQLTagItem> = Vec::new();

        for token in self.tokens {
            match token {
                QueryToken::SymbolToken{ symbol, value: _ } => {
                    // only one tag in stack stack, and current can't be 'LeftAttrBracket'
                    if tag_stack.len() == 1 && *symbol != TokenSymbol::LeftAttrBracket {
                        let item = tag_stack.last().unwrap().clone();

                        match ops_stack.last().unwrap().0 {
                            TokenSymbol::Include => {
                                self.builder.add_include(item);
                                tag_stack.pop();
                                ops_stack.pop();
                            },
                            TokenSymbol::Exclude => {
                                self.builder.add_exclude(item);
                                tag_stack.pop();
                                ops_stack.pop();
                            },
                            _ => {},
                        }
                    }
                    // if match the 'Right Group Bracket', start popup tags
                    else if *symbol == TokenSymbol::RightGroupBracket {
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
                                self.builder.add_group(StringQLGroupPrefix::Include, tags);
                                ops_stack.pop();
                            },
                            TokenSymbol::Exclude => {
                                self.builder.add_group(StringQLGroupPrefix::Exclude, tags);
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
                QueryToken::AttributeToken { symbol: _, value } => {
                    dbg!(&tag_stack);
                    if let Some(top_tag) = tag_stack.last_mut() {
                        top_tag.set_attribute(value.to_string());
                    }
                },
                QueryToken::TagToken { symbol: _, namespace: _, value } => {
                    let tag_id = self.tag_id_map.get(value).unwrap();
                    tag_stack.push(StringQLTagItem::new(tag_id.to_string(), None));
                },
                _ => {}
            }
        };

        Ok(self.builder.build())
    }
}