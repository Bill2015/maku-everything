use crate::resource::domain::{ResourceError, ResourceGenericError};

use super::types::TokenSymbol;
use super::token::QueryToken;

macro_rules! syntax_err {
    ($msg: literal) => {
        Err(
            ResourceError::QueryingByString(
                ResourceGenericError::InvalidQueryingString { 
                    message: $msg.to_string()
                }
            )
        )
    };
}
// Syntax Tree
// entry |= express + $
// express |= prefix + express_body
// express_body |= tagname
//              |= bracket + express_tags + bracket
// express_tags |= tagname

pub struct Syntax<'a> {
    tokens: &'a Vec<QueryToken>,
    current: usize,
}

impl<'a> Syntax<'a> {
    pub fn new(tokens: &'a Vec<QueryToken>) -> Self {
        Self { tokens, current: 0 }
    }

    fn peek(&self) -> Option<&QueryToken> {
        self.tokens.get(self.current)
    }

    fn check_tagname(&self, value: &String) -> Result<bool, ResourceError> {
        if value.is_empty() {
            return syntax_err!("The 'Tag Name' is empty");
        }
        return Ok(true);
    }

    fn match_token(&self, target: TokenSymbol) -> bool {
        if self.peek().is_none() {
            return false
        }
        match self.peek().unwrap() {
            QueryToken::SymbolToken{ symbol, value: _ } => *symbol == target,
            QueryToken::AttributeToken{ symbol, value: _ } => *symbol == target,
            QueryToken::TagToken{ symbol, namespace: _, value: _ } => *symbol == target,
            _ => false,
        }
    }

    fn comuse_token(&mut self) {
        self.current += 1;
    }

    fn eof(&self) -> bool {
        self.match_token(TokenSymbol::EOF)
    }

    fn express_attribute(&mut self) -> Result<(), ResourceError> {
        if self.match_token(TokenSymbol::LeftAttrBracket) == false {
            return Ok(())
        }
        
        self.comuse_token();
        if let Some(QueryToken::AttributeToken{ symbol: _, value: _ }) = self.peek() {
            self.comuse_token();
            if self.match_token(TokenSymbol::RightAttrBracket) {
                self.comuse_token();
                return Ok(());
            }
            else {
                return syntax_err!("Attribute missing '}'");
            }
        }
        else {
            return syntax_err!("Attribute can't be empty");
        }
        
    }

    fn express_tags(&mut self) -> Result<(), ResourceError> {
        let mut tag_count = 0;
        loop {
            if let Some(QueryToken::TagToken{ symbol: _, namespace: _, value }) = self.peek() {
                self.check_tagname(value)?;
                self.comuse_token();
                self.express_attribute()?;
                tag_count += 1;
            }
            else if self.match_token(TokenSymbol::RightGroupBracket) {
                break;
            }
            else {
                return syntax_err!("Brackets inside can only contain 'Tag Name'");
            }
        }
        if tag_count < 2 {
            return syntax_err!("Brackets inside it must be contain at least 2 'Tag Name'");
        }

        Ok(())
    }

    fn express_body(&mut self) -> Result<(), ResourceError> {
        if let Some(QueryToken::TagToken{ symbol: _, namespace: _, value }) = self.peek() {
            self.check_tagname(value)?;
            self.comuse_token();
            self.express_attribute()?;
            Ok(())
        }
        else if self.match_token(TokenSymbol::LeftGroupBracket) {
            self.comuse_token();
            self.express_tags()?;
            self.comuse_token();
            Ok(())
        }
        else {
            syntax_err!("After '+' or '-' it must be a 'Tag Name' or '['")
        }
    }

    fn express(&mut self) -> Result<(), ResourceError> {
        if self.match_token(TokenSymbol::Include) || self.match_token(TokenSymbol::Exclude) {
            self.comuse_token();
            self.express_body()?;
            Ok(())
        }
        else {
            syntax_err!("It must be start with '+' or '-' first")
        }
    }
    

    pub fn check(&mut self) -> Result<String, ResourceError> {
        while !self.eof() {
            self.express()?;
        }
        Ok("good string".to_string())
    }
}
