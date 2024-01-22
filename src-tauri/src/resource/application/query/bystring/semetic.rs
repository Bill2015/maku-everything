use crate::{tag::{repository::TagQueryRepository, infrastructure::TagQueryBuilder}, resource::{infrastructure::{SystemTag, AttributeValue}, domain::{ResourceError, ResourceGenericError}}};
use super::token::QueryToken;

pub struct Semantic<'a>  {
    tokens: Vec<QueryToken>,
    repo: &'a TagQueryRepository<'a>,
}

impl<'a> Semantic<'a> {
    pub fn new(tokens: &Vec<QueryToken>, repo: &'a TagQueryRepository) -> Self {
        Self {
            repo,
            tokens: tokens.iter().cloned().collect(),
        }
    }

    fn peek_attribute(&self, index: usize) -> Option<String> {
        if let Some(val) = self.tokens.get(index) {
            return match val {
                QueryToken::AttributeToken { value, .. } => Some(value.clone()),
                _ => None,
            }
        }
        None
    }

    fn parse_value(&mut self) -> Result<(), ResourceError> {
        let mut new_tokens: Vec<QueryToken> = Vec::new();
        let mut current: usize = 0;

        while current < self.tokens.len() {
            let token = self.tokens.get(current).unwrap();

            match token {
                QueryToken::SymbolToken { symbol, .. } => {
                    match symbol {
                        super::types::TokenSymbol::Attribute => {},
                        super::types::TokenSymbol::LeftAttrBracket => {},
                        super::types::TokenSymbol::RightAttrBracket => {},
                        _ => new_tokens.push(token.clone())
                    }
                },
                QueryToken::AttributeToken { .. } => {},
                QueryToken::TagToken { .. } => {
                    new_tokens.push(token.clone())
                },
                QueryToken::SystemTagToken { namespace, value, .. } => {
                    let attribute = self.peek_attribute(current + 2);

                    let mut new_token = token.clone();

                    if attribute.is_some() {
                        if let Ok(attr_type) = SystemTag::attr_type(&namespace, &value) {
                    
                            let new_attrval = AttributeValue::parse_from(&attribute.unwrap(), attr_type)
                                .or(Err(ResourceError::QueryingByString(ResourceGenericError::InvalidQueryingString{ message: "Attribute invalid".to_string() })))?;
        
                            new_token.set_attribute(new_attrval);                           
                        } 
                    }
                    new_tokens.push(new_token);
                },
            }

            current += 1;
        }

        self.tokens = new_tokens;
        Ok(())
    }

    async fn generate_tag_id(&mut self) -> Result<(), ResourceError> {
        for token in self.tokens.iter_mut() {
            if let QueryToken::TagToken { namespace, value, .. } = token {
                let mut builder = TagQueryBuilder::new()
                    .set_name(value.to_string());

                if let Some(namepace) = namespace {
                    builder = builder.set_belong_subject_name(namepace.to_string());
                }

                let result = &self.repo.query(builder)
                    .await
                    .or(Err(ResourceError::QueryingByString(ResourceGenericError::DBInternalError())))?;

                // find multiple same name tags
                let _ = match result.len() {
                    1 => {
                        let result = result.first().unwrap();
                        token.set_tag_id(result.id.to_string());
                        Ok(())
                    },
                    0 => Err(ResourceError::QueryingByString(ResourceGenericError::TagNotExists())),
                    _ => Err(ResourceError::QueryingByString(ResourceGenericError::FindAmbiguousTags()))
                }?;
            }
        }

        Ok(())
    }

    pub async fn parse(&mut self) -> Result<Vec<QueryToken>, ResourceError> {
        self.generate_tag_id().await?;
        self.parse_value()?;

        Ok(self.tokens.clone())
    }
}