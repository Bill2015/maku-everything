use std::collections::HashMap;

use crate::resource::domain::ResourceError;

use super::types::{QueryToken, TokenSymbol};

#[derive(Debug)]
pub enum SQLGroupPrefixType {
    Include,
    Exclude,
}

#[derive(Debug)]
pub struct SqlGroup {
    pub prefix: SQLGroupPrefixType,
    pub items: Vec<String>,
}

#[derive(Debug)]
pub struct SqlObjectData {
    excludes: Vec<String>,
    includes: Vec<String>,
    groups: Vec<SqlGroup>,
}

impl SqlObjectData {
    pub fn new() -> Self {
        Self { excludes: Vec::new(), includes: Vec::new(), groups: Vec::new() }
    }
    
    fn add_exclude(&mut self, item: String) {
        self.excludes.push(item);
    }

    fn add_include(&mut self, item: String) {
        self.includes.push(item);
    }

    fn add_group(&mut self, item: SqlGroup) {
        self.groups.push(item);
    }

    pub fn get_excludes(&self) -> &Vec<String> {
        &self.excludes
    }

    pub fn get_includes(&self) -> &Vec<String>{
        &self.includes
    }

    pub fn get_groups(&self) ->  &Vec<SqlGroup> {
        &self.groups
    }
    
}


pub struct SQLQueryObjectGenerator<'a> {
    tokens: &'a Vec<QueryToken>,
    tag_id_map: HashMap<String, String>,
}

impl<'a> SQLQueryObjectGenerator<'a> {
    pub fn new(tokens: &'a Vec<QueryToken>, tag_id_map: HashMap<String, String>) -> Self {
        Self { tokens, tag_id_map }
    }

    pub fn gen(&self) -> Result<SqlObjectData, ResourceError> {

        let mut sqldata = SqlObjectData::new();

        let mut stack: Vec<&QueryToken> = Vec::new();

        for token in self.tokens {
            if (token.symbol == TokenSymbol::Include) || (token.symbol == TokenSymbol::Exclude) {
                stack.push(&token);
            }
            else if token.symbol == TokenSymbol::LeftBracket {
                stack.push(&token);
            }
            else if token.symbol == TokenSymbol::RightBracket {
                let mut group_item: Vec<String> = Vec::new();

                while let Some(top_token) = stack.last() {
                    match top_token.symbol {
                        TokenSymbol::Exclude => {
                            sqldata.add_group(SqlGroup { prefix: SQLGroupPrefixType::Exclude, items: group_item });
                            break;
                        },
                        TokenSymbol::Include => {
                            sqldata.add_group(SqlGroup { prefix: SQLGroupPrefixType::Include, items: group_item });
                            break;
                        },
                        TokenSymbol::LeftBracket => {},
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
                        sqldata.add_include(tag_id.to_string());
                        stack.pop();
                    },
                    TokenSymbol::Exclude => {
                        sqldata.add_exclude(tag_id.to_string());
                        stack.pop();
                    },
                    _ => {
                        stack.push(&token);
                    }
                }
            }
        };

        Ok(sqldata)
    }
}