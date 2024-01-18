use std::collections::HashMap;

use crate::resource::infrastructure::ResourceQueryBuilder;
use crate::tag::repository::TagQueryRepository;
use crate::tag::infrastructure::TagQueryBuilder;
use crate::resource::domain::{ResourceError, ResourceGenericError};

use super::types::{QueryToken, TokenSymbol, QueryingStringSymbol};

 
pub struct SQLQueryGenerator<'a> {
    tokens: &'a Vec<QueryToken>,
    tag_id_map: HashMap<String, String>,
    sql_string: String,
}

impl<'a> SQLQueryGenerator<'a> {
    pub fn new(tokens: &'a Vec<QueryToken>) -> Self {
        Self { 
            tokens,
            tag_id_map: HashMap::new(),
            sql_string: "".to_string() 
        }
    }

    pub async fn preprocessor(&mut self, tag_repo: &TagQueryRepository<'_>) -> Result<(), ResourceError> {
        for token in self.tokens.iter() {
            if token.token_name != TokenSymbol::TagName {
                continue;
            }

            let tokenval = &token.value;
            let mut builder = TagQueryBuilder::new();

            // have subject name
            if let Some(index) = tokenval.chars().position(|x| QueryingStringSymbol::SubjectDelimiter == x) {
                // for unicode text
                let unicode_index = tokenval
                    .char_indices()
                    .map(|(i, _)| i)
                    .nth(index)
                    .unwrap();
                let subject_name = tokenval[0..unicode_index].to_string();
                let tag_name = tokenval[unicode_index + 1..tokenval.len()].to_string();

                builder = builder
                    .set_name(tag_name)
                    .set_belong_subject_name(subject_name);
            }
            // only tag name
            else {
                builder = builder.set_name(tokenval.to_string());
            }

            let result = tag_repo.query(builder).await
                .or(Err(ResourceError::QueryingByString(ResourceGenericError::DBInternalError())))?;
        
            // find multiple same name tags
            if result.len() >= 2 {
                return Err(ResourceError::QueryingByString(ResourceGenericError::FindAmbiguousTags()));
            }

            if let Some(element) = result.get(0) {
                self.tag_id_map.insert(tokenval.to_string(), element.id.to_string());
            }
            else {
                return Err(ResourceError::QueryingByString(ResourceGenericError::TagNotExists()));
            }
        }

        Ok(())
    }

    pub fn gen(&self) {
        let mut tag_stack: Vec<&QueryToken> = Vec::new();
        let mut ops_stack: Vec<&QueryToken> = Vec::new();

        let mut builder = ResourceQueryBuilder::new();

        for token in self.tokens {


            if (token.token_name == TokenSymbol::Include) || (token.token_name == TokenSymbol::Exclude) {
                ops_stack.push(&token);
            }
            else if token.token_name == TokenSymbol::LeftBracket {
                ops_stack.push(&token);
            }
            else if token.token_name == TokenSymbol::TagName {
                if ops_stack.last().is_some_and(|x| x.token_name == TokenSymbol::LeftBracket) {
                    tag_stack.push(&token);
                }
                else {
                    let prefix_op = ops_stack.pop().unwrap();
                    let tag_id = self.tag_id_map.get(&token.value).unwrap();
                    if prefix_op.token_name == TokenSymbol::Include {
                        builder = builder.add_include_tag(tag_id.to_string());
                    }
                    else if prefix_op.token_name == TokenSymbol::Exclude {
                        builder = builder.add_exclude_tag(tag_id.to_string());
                    }
                }
            }
        }

        dbg!(builder.build());
    }
}