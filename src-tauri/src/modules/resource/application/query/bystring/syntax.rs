use crate::resource::domain::ResourceGenericError;

use super::types::TokenSymbol;
use super::token::QueryToken;

macro_rules! syntax_err {
    ($msg: literal) => {
        Err(
            ResourceGenericError::InvalidQueryingString { 
                message: $msg.to_string()
            }
        )
    };
}

/// ## Syntax Tree
/// ```r
///             entry |= express + $
/// 
///           express |= prefix + express_body
/// 
///      express_body |= normal tag + express_attribute?
///                   |= system tag + express_attribute?
///                   |= [ express_tags ]
/// 
///      express_tags |= normal tag + express_attribute?
///                   |= system tag + express_attribute?
/// 
/// express_attribute |= { content }
/// ```
///
pub struct StringQLSyntaxChecker<'a> {
    tokens: &'a Vec<QueryToken>,

    current: usize,
}

impl<'a> StringQLSyntaxChecker<'a> {
    pub fn new(tokens: &'a Vec<QueryToken>) -> Self {
        Self { tokens, current: 0 }
    }

    fn peek(&self) -> Option<&QueryToken> {
        self.tokens.get(self.current)
    }

    fn check_tagname(&self, value: &String) -> Result<bool, ResourceGenericError> {
        if value.is_empty() {
            return syntax_err!("The 'Tag Name' is empty");
        }
        return Ok(true);
    }

    fn match_token(&self, target: TokenSymbol) -> bool {
        match self.peek() {
            Some(QueryToken::SymbolToken{ symbol, .. }) => *symbol == target,
            Some(QueryToken::AttributeToken{ symbol, .. }) => *symbol == target,
            Some(QueryToken::TagToken{ symbol, .. }) => *symbol == target,
            Some(QueryToken::SystemTagToken { symbol, .. }) => *symbol == target,
            None => false,
        }
    }

    fn comuse_token(&mut self) {
        self.current += 1;
    }

    fn eof(&self) -> bool {
        self.match_token(TokenSymbol::EOF)
    }

    fn express_attribute(&mut self) -> Result<(), ResourceGenericError> {
        if self.match_token(TokenSymbol::LeftAttrBracket) == false {
            return Ok(())
        }
        
        self.comuse_token();
        if let Some(QueryToken::AttributeToken{ .. }) = self.peek() {
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

    fn express_tags(&mut self) -> Result<(), ResourceGenericError> {
        let mut tag_count = 0;
        loop {
            if let Some(QueryToken::TagToken{ value, .. }) = self.peek() {
                self.check_tagname(value)?;
                self.comuse_token();
                self.express_attribute()?;
                tag_count += 1;
            }
            else if let Some(QueryToken::SystemTagToken{ value, .. }) = self.peek() {
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

    fn express_body(&mut self) -> Result<(), ResourceGenericError> {
        if let Some(QueryToken::TagToken{ value, .. }) = self.peek() {
            self.check_tagname(value)?;
            self.comuse_token();
            self.express_attribute()?;
            Ok(())
        }
        else if let Some(QueryToken::SystemTagToken{ value, .. }) = self.peek() {
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

    fn express(&mut self) -> Result<(), ResourceGenericError> {
        if self.match_token(TokenSymbol::Include) || self.match_token(TokenSymbol::Exclude) {
            self.comuse_token();
            self.express_body()?;
            Ok(())
        }
        else {
            syntax_err!("It must be start with '+' or '-' first")
        }
    }
    

    pub fn check(&mut self) -> Result<String, ResourceGenericError> {
        while !self.eof() {
            self.express()?;
        }
        Ok("good string".to_string())
    }
}
