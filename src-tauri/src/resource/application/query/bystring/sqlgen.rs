use std::collections::HashMap;

use crate::resource::domain::ResourceError;
use crate::resource::infrastructure::{StringQLGroupPrefix, StringQLObject, StringQLObjectBuilder};

use super::types::{QueryToken, TokenSymbol};

pub struct SQLQueryObjectGenerator<'a> {
    tokens: &'a Vec<QueryToken>,
    /** For tag id mapping  */
    tag_id_map: HashMap<String, String>,
}

impl<'a> SQLQueryObjectGenerator<'a> {
    pub fn new(tokens: &'a Vec<QueryToken>, tag_id_map: HashMap<String, String>) -> Self {
        Self { tokens, tag_id_map }
    }

    pub fn gen(&self) -> Result<StringQLObject, ResourceError> {

        let mut builder = StringQLObjectBuilder::new();

        let mut stack: Vec<&QueryToken> = Vec::new();

        for token in self.tokens {
            if (token.symbol == TokenSymbol::Include) || (token.symbol == TokenSymbol::Exclude) {
                stack.push(&token);
            }
            else if token.symbol == TokenSymbol::LeftGroupBracket {
                stack.push(&token);
            }
            else if token.symbol == TokenSymbol::RightGroupBracket {
                let mut group_item: Vec<String> = Vec::new();

                while let Some(top_token) = stack.last() {
                    match top_token.symbol {
                        TokenSymbol::Exclude => {
                            builder = builder.add_group(StringQLGroupPrefix::Exclude, group_item);
                            break;
                        },
                        TokenSymbol::Include => {
                            builder = builder.add_group(StringQLGroupPrefix::Include, group_item);
                            break;
                        },
                        TokenSymbol::LeftGroupBracket => {},
                        _ => {
                            let tag_id = self.tag_id_map.get(&top_token.value).unwrap();
                            group_item.push(tag_id.to_string());
                        }
                    }
                    stack.pop();
                }
            }
            else if token.symbol == TokenSymbol::TagName {
                let tag_id = self.tag_id_map.get(&token.value).unwrap();

                match stack.last().unwrap().symbol {
                    TokenSymbol::Include => {
                        builder = builder.add_include(tag_id.to_string());
                        stack.pop();
                    },
                    TokenSymbol::Exclude => {
                        builder = builder.add_exclude(tag_id.to_string());
                        stack.pop();
                    },
                    _ => {
                        stack.push(&token);
                    }
                }
            }
        };

        Ok(builder.build())
    }
}