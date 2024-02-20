use crate::modules::common::infrastructure::QueryBuilder;
use crate::modules::tag::application::dto::TagAttrDto;
use crate::modules::tag::repository::TagQueryRepository;
use crate::modules::tag::infrastructure::TagQueryBuilder;
use crate::modules::resource::infrastructure::{AttributeValue, AttributeValueType, SystemTag};
use crate::modules::resource::domain::ResourceGenericError;

use super::token::QueryToken;
use super::types::TokenSymbol;

macro_rules! block {
    ($xs:block) => {
        loop { break $xs }
    };
}

macro_rules! semantic_err {
    ($msg: expr) => {
        ResourceGenericError::InvalidQueryingString { message: $msg.to_string() }
    };
}

pub struct StringQLSemantic<'a>  {
    tokens: Vec<QueryToken>,

    repo: &'a TagQueryRepository<'a>,

    belong_category: &'a Option<String>,
}

impl<'a> StringQLSemantic<'a> {
    pub fn new(tokens: &Vec<QueryToken>, belong_category: &'a Option<String>, repo: &'a TagQueryRepository) -> Self {
        Self {
            repo,
            belong_category,
            tokens: tokens.iter().cloned().collect(),
        }
    }

    fn peek_attribute(&self, index: usize) -> Option<String> {
        match self.tokens.get(index) {
            Some(QueryToken::AttributeToken { value, .. }) => Some(value.clone()),
            _ => None,
        }
    }

    fn parse_value(&mut self) -> Result<(), ResourceGenericError> {
        let mut new_tokens: Vec<QueryToken> = Vec::new();
        let mut current: usize = 0;

        while current < self.tokens.len() {
            let token = self.tokens.get(current).unwrap();

            match token {
                QueryToken::SymbolToken { symbol, .. } => {
                    match symbol {
                        TokenSymbol::Attribute => {},
                        TokenSymbol::LeftAttrBracket => {},
                        TokenSymbol::RightAttrBracket => {},
                        _ => new_tokens.push(token.clone())
                    }
                },
                QueryToken::TagToken { attrtype, .. }  => block!({ // NOTE: this is early breakout the match
                    if *attrtype == AttributeValueType::None {
                        new_tokens.push(token.clone());
                        break;
                    }

                    if let Some(attr) = self.peek_attribute(current + 2) {
                        let mut new_token = token.clone();

                        let new_attrval = AttributeValue::parse_from(attr, *attrtype)
                            .map_err(|err| semantic_err!(err.to_string()))?;

                        new_token.set_attribute(new_attrval);
                        new_tokens.push(new_token);
                        break;
                    }
                    // all tag's attribute are optional
                    else {
                        new_tokens.push(token.clone());
                    }
                }),
                QueryToken::SystemTagToken { attrtype, .. } => {
                    let mut new_token = token.clone();

                    // peek next 2 token, if it is attribute
                    // syntax check will make this to be true
                    if let Some(attr) = self.peek_attribute(current + 2) {
                        let new_attrval = AttributeValue::parse_from(attr, *attrtype)
                            .map_err(|err| semantic_err!(err.to_string()))?;

                        new_token.set_attribute(new_attrval);   
                    }
                    else if *attrtype != AttributeValueType::OptionText {
                        return Err(semantic_err!("Attribute can't be empty"));
                    }
                    
                    new_tokens.push(new_token);
                },
                // skip it, because `self.peek_attribute(current + 2)`
                QueryToken::AttributeToken { .. } => { },
            }

            current += 1;
        }

        self.tokens = new_tokens;
        Ok(())
    }

    async fn generate_tag_id_and_attr_type(&mut self) -> Result<(), ResourceGenericError> {
        for token in self.tokens.iter_mut() {
            match token {
                QueryToken::SystemTagToken { namespace, value, .. } => {
                    if let Ok(attr_type) = SystemTag::attr_type(namespace.as_str(), value.as_str()) {
                        token.set_attribute_type(attr_type);
                    }
                },
                QueryToken::TagToken { namespace, value, .. } => {
                    let mut builder = TagQueryBuilder::new()
                        .set_name(value.to_string());
    
                    if let Some(namepace) = namespace {
                        builder = builder.set_belong_subject_name(namepace.to_string());
                    }
    
                    if let Some(category) = self.belong_category {
                        builder = builder.set_belong_category(category);
                    }
    
                    let builder_result = builder.build()
                        .map_err(|err| ResourceGenericError::InvalidQueryingString { message: err.to_string() })?;
    
                    let result = &self.repo.query(builder_result)
                        .await
                        .or(Err(ResourceGenericError::DBInternalError()))?;
    
                    // find multiple same name tags
                    let _ = match result.len() {
                        1 => {
                            let result = result.first().unwrap();
                            token.set_tag_id(result.id.to_string());
                            
                            let attr_type = match result.attrval {
                                TagAttrDto::Normal => AttributeValueType::None,
                                TagAttrDto::Number { .. } => AttributeValueType::NumberRange,
                                TagAttrDto::Text { .. } => AttributeValueType::Text,
                                TagAttrDto::Date { .. } => AttributeValueType::DateRange,
                                TagAttrDto::Bool { .. } => AttributeValueType::Bool,
                            };
                            token.set_attribute_type(attr_type);
    
                            Ok(())
                        },
                        0 => Err(ResourceGenericError::TagNotExists()),
                        _ => Err(ResourceGenericError::FindAmbiguousTags()),
                    }?;
                },
                _ => { },
            }

        }

        Ok(())
    }

    pub async fn parse(&mut self) -> Result<Vec<QueryToken>, ResourceGenericError> {
        self.generate_tag_id_and_attr_type().await?;
        self.parse_value()?;

        Ok(self.tokens.clone())
    }
}