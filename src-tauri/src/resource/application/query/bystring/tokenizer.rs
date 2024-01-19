use std::str::FromStr;

use super::types::QueryToken;
use super::types::QueryingStringSymbol;
use super::types::TokenSymbol;

pub struct Tokenizer<'a> {
    query_string: &'a str,
    current: usize,
    char_num: usize,
    tokens: Vec<QueryToken>,
}

impl<'a> Tokenizer<'a> {
    
    pub fn new(query_string: &'a str) -> Self {
        Self {
            query_string,
            current: 0,
            char_num: query_string.chars().count(),
            tokens: Vec::new(),
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
        if let Some(index) = value.chars().position(|x| QueryingStringSymbol::SubjectDelimiter == x) {
            // for unicode text
            let unicode_index = value
                .char_indices()
                .map(|(i, _)| i)
                .nth(index)
                .unwrap();
            let subject_name = value[0..unicode_index].to_string();
            let value = value[unicode_index + 1..value.len()].to_string();

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

    fn scan_tag_name(&mut self) -> String {
        let mut chars: Vec<char> = vec![self.peek_prev().unwrap_or('\0')];
        let quoted = self.peek_prev().unwrap() == '"';
    
        while !self.is_end() {
            let ch = self.peek();
            let symbol = TokenSymbol::from_str(&ch.to_string());
            if symbol.is_ok() || (quoted == false && ch.is_whitespace()) {
                break;
            }
            else if ch == '"' {
                chars.push(ch);
                self.next_ch();
                break;
            }
            else {
                chars.push(ch);
                self.next_ch();
            }
        }

        chars.iter()
            .filter(|c| **c != '"')
            .collect::<String>()
    }

    fn scan(&mut self) {
        while !self.is_end() {
            let ch = self.next_ch();

            // general symbol
            if let Ok(symbol) = TokenSymbol::from_str(&ch.to_string()) {
                self.add_token(QueryToken::new(symbol, None, ch.to_string()));
            }
            else if ch.is_whitespace() {
                // do nothing
            }
            else {
                let tag_name = self.scan_tag_name();
                let (subject_name, tag_name) = self.separate_namespace(tag_name);
                self.add_token(QueryToken::new(TokenSymbol::TagName, subject_name, tag_name))
            }
        }
    }

    pub fn parse(&mut self) -> Vec<QueryToken> {
        self.scan();
        self.tokens.clone()
    }
}
