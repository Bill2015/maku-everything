use crate::modules::resource::infrastructure::SystemTag;
use crate::utils::StringUtils;

use super::types::QueryingStrChar;
use super::types::TokenSymbol;
use super::token::QueryToken;

pub struct StringQLTokenizer {
    query_string: String,

    current: usize,

    char_num: usize,

    tokens: Vec<QueryToken>,
}

impl StringQLTokenizer {
    
    pub fn new<S: Into<String>>(query_string: S) -> Self {
        // add EOF symbol
        let s: String = format!("{}$", query_string.into().trim().to_string());
        Self {
            current: 0,
            tokens: Vec::new(),
            char_num: s.chars().count(),
            query_string: s,
        }
    }

    ///
    /// Separate namespace \
    /// 
    /// Example: \
    /// ```
    /// separate_namespace("Language:Typescript") => ("Language", "Typescript")
    /// separate_namespace("Typescript") => (None, "Typescript")
    /// ```
    fn separate_namespace(&self, value: String) -> (Option<String>, String) {
        if let Some(index) = value.chars().position(|x| QueryingStrChar::NamespaceDelimiter == x) {
            let subject_name = value.slice(0..index).trim().to_string();
            let value = value.slice(index + 1..value.len()).trim().to_string();

            return (Some(subject_name), value);
        }
        
        (None, value)
    }

    fn is_end(&self) -> bool {
        self.current >= self.char_num
    }

    fn next_ch(&mut self) -> char {
        let ch = self.peek();
        self.current += 1;
        ch
    }

    fn peek_prev(&self) -> Option<char> {
        self.query_string.chars().nth(self.current.checked_sub(1)?)
    }

    fn peek(&self) -> char {
        match self.is_end() {
            true => '\0',
            false => self.query_string.chars().nth(self.current).unwrap()
        }
    }

    fn add_token(&mut self, token: QueryToken) {
        self.tokens.push(token);
    }

    fn scan_tag_name(&mut self) {
        let mut chars: Vec<char> = vec![self.peek_prev().unwrap_or('\0')];
        let quoted = QueryingStrChar::StringWrapper == self.peek_prev().unwrap();
    
        while !self.is_end() {
            let ch = self.peek();
            let symbol = TokenSymbol::from_str(&ch.to_string());
            if symbol.is_ok() || (quoted == false && ch.is_whitespace()) {
                break;
            }
            else if QueryingStrChar::StringWrapper == ch {
                self.next_ch();
                break;
            }
            else {
                chars.push(ch);
                self.next_ch();
            }
        }

        let tag_val = chars.into_iter()
            .filter(|c| QueryingStrChar::StringWrapper != *c )
            .collect::<String>();
        let is_system = SystemTag::is_defined(&tag_val);

        let (namespace, name) = self.separate_namespace(tag_val);

        match is_system {
            true => self.add_token(QueryToken::new_system_tag(TokenSymbol::TagName, namespace.clone().unwrap(), name)),
            false => self.add_token(QueryToken::new_tag(TokenSymbol::TagName, namespace, name)),
        }
    }

    fn scan_attribute(&mut self) {
        let mut chars: Vec<char> = Vec::new();

        while !self.is_end() {
            let ch = self.peek();
            if TokenSymbol::from_str(&ch.to_string()).is_ok() {
                let attribute_val = chars.iter().collect::<String>();
                self.add_token(QueryToken::new_attribute(TokenSymbol::Attribute, attribute_val));
                break;
            }
            else {
                chars.push(ch);
                self.next_ch();
            }
        }
    }

    fn scan(&mut self) {
        while !self.is_end() {
            let ch = self.next_ch();

            // general symbol
            if let Ok(symbol) = TokenSymbol::from_str(&ch.to_string()) {
                self.add_token(QueryToken::new_symbol(symbol.clone(), ch.to_string()));

                if symbol == TokenSymbol::LeftAttrBracket {
                    self.scan_attribute();
                }
            }
            else if ch.is_whitespace() {
                // do nothing
            }
            else {
                self.scan_tag_name();
            }
        }
    }

    pub fn parse(&mut self) -> Vec<QueryToken> {
        self.scan();
        self.tokens.clone()
    }
}
