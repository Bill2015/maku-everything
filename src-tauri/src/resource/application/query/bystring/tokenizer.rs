use std::str::FromStr;

use super::types::QueryToken;
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

    fn is_end(&self) -> bool {
        self.current >= self.char_num
    }

    fn next_ch(&mut self) -> char {
        let ch = self.peek();
        self.current += 1;
        ch
    }

    fn peek_prev(&self) -> Option<char> {
        if self.current <= 0 {
           return None;
        }
        Some(self.query_string.chars().nth(self.current - 1).unwrap())
    }

    fn peek(&self) -> char {
        if self.is_end() {
            '\0'
        }
        else {
            self.query_string.chars().nth(self.current).unwrap()
        }
    }

    fn add_token(&mut self, token: QueryToken) {
        self.tokens.push(token);
    }

    fn scan_tag_name(&mut self) -> String {
        let mut chars: Vec<char> = Vec::new();

        let prev_ch = self.peek_prev().unwrap();
        let quoted = prev_ch == '"';
        chars.push(prev_ch);  
    
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
            .map(|c| if *c == '"' { "".to_string() } else { c.to_string() } )
            .collect::<Vec<String>>()
            .join("")
    }

    fn scan(&mut self) {
        let ch = self.next_ch();

        if let Ok(symbol) = TokenSymbol::from_str(&ch.to_string()) {
            self.add_token(QueryToken::new(ch.to_string(), symbol));
        }
        else if ch.is_whitespace() {
            // do nothing
        }
        else {
            let tag_name = self.scan_tag_name();
            self.add_token(QueryToken::new(tag_name, TokenSymbol::TagName))
        }
    }

    pub fn parse(&mut self) -> Vec<QueryToken> {
        while !self.is_end() {
            self.scan();
        }

        self.tokens.clone()
    }

}
