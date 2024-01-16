use crate::resource::domain::{ResourceError, ResourceGenericError};

use super::types::{QueryToken, TokenSymbol};

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

    fn match_token(&self, symbol: TokenSymbol) -> bool {
        let token = self.peek();
        if token.is_some_and(|val| val.token_name == symbol) {
            true
        }
        else {
            false
        }
    }

    fn comuse_token(&mut self) {
        self.current += 1;
    }

    fn eof(&self) -> bool {
        self.match_token(TokenSymbol::EOF)
    }

    fn express_tags(&mut self) -> Result<(), ResourceError> {
        let mut tag_count = 0;
        while true {
            if self.match_token(TokenSymbol::TagName) {
                self.comuse_token();
                tag_count += 1;
            }
            else if self.match_token(TokenSymbol::RightBracket) {
                break;
            }
            else {
                return syntax_err!("Brackets inside can only contain 'Tag Name'")
            }
        }
        if tag_count < 2 {
            return syntax_err!("Brackets inside it must be contain at least 2 'Tag Name'")
        }

        Ok(())
    }

    fn express_body(&mut self) -> Result<(), ResourceError> {
        if self.match_token(TokenSymbol::TagName) {
            self.comuse_token();
            Ok(())
        }
        else if self.match_token(TokenSymbol::LeftBracket) {
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
