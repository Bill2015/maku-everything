use std::iter::Peekable;
use std::str::{FromStr, Chars};

use async_trait::async_trait;

use crate::common::application::IQueryHandler;
use crate::resource::application::dto::ResourceResDto;
use crate::resource::domain::{ResourceError, ResourceGenericError};
use crate::resource::repository::ResourceQueryRepository;

pub struct StringResourceQuery {
    pub query_string: String,
}


pub struct StringResourceHandler<'a> {
    resource_repo: &'a ResourceQueryRepository<'a>,
}

impl<'a> StringResourceHandler<'a> {
    pub fn register(resource_repo: &'a ResourceQueryRepository) -> Self {
        Self { resource_repo: &resource_repo }
    }
}

#[async_trait]
impl IQueryHandler<StringResourceQuery> for StringResourceHandler<'_>{
    fn get_name() -> String {
        String::from("String Querying Resource")
    }

    type Output = Result<Vec<ResourceResDto>, ResourceError>;

    async fn query(&self, query: StringResourceQuery) -> Self::Output {
        let StringResourceQuery { query_string } = query;

        dbg!("========================================");
        let q = format!("{}$", query_string.trim());
        dbg!(&q);

        let mut tokenizer = Tokenizer::new(&q);
        let res = tokenizer.parse();
        dbg!(res);
        // let tokens = tokenizer(query_string.trim().to_string());
        // dbg!(tokens);
        // let result = self.resource_repo
        //     .query(query_builder)
        //     .await;
    
        Err(ResourceError::Query(ResourceGenericError::DBInternalError()))
        // match result {
        //     Ok(value) => Ok(value),
        //     _ => Err(ResourceError::Query(ResourceGenericError::DBInternalError())),
        // }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum TokenSymbol {
    TagName,
    Include,
    Exclude,
    EOF,
    LeftBracket,
    RightBracket,
}

impl TokenSymbol {
    fn as_str(&self) -> &str {
        match self {
            Self::TagName => "Tag Name",
            Self::Include => "+",
            Self::Exclude => "-",
            Self::LeftBracket => "[",
            Self::RightBracket => "]",
            Self::EOF => "$",
        }
    }
}

impl FromStr for TokenSymbol {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "+" => Ok(Self::Include),
            "-" => Ok(Self::Exclude),
            "[" => Ok(Self::LeftBracket),
            "]" => Ok(Self::RightBracket),
            "$" => Ok(Self::EOF),
            _ => Err(String::from("Not Match Operator Symbol")) 
        }
    }
}

#[derive(Debug, Clone)]
struct QueryToken {
    value: String,
    token_name: TokenSymbol,
}

struct Tokenizer<'a> {
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

    fn scan_tag_name(&mut self) {
        let mut ch_vec: Vec<char> = Vec::new();

        let prev_ch = self.peek_prev().unwrap();
        let quoted = prev_ch == '"';
        ch_vec.push(prev_ch);  
    
        while !self.is_end() {
            let ch = self.peek();
            let symbol = TokenSymbol::from_str(&ch.to_string());
            if symbol.is_ok() || (quoted == false && ch.is_whitespace()) {
                break;
            }
            else if ch == '"' {
                ch_vec.push(ch);
                self.next_ch();
                break;
            }
            else {
                ch_vec.push(ch);
                self.next_ch();
            }
        }

        self.add_token(QueryToken {
            value: ch_vec
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(""),
            token_name: TokenSymbol::TagName,
        });
    }

    fn scan(&mut self) {
        let ch = self.next_ch();

        if let Ok(symbol) = TokenSymbol::from_str(&ch.to_string()) {
            self.add_token(QueryToken { 
                value: ch.to_string(), 
                token_name: symbol 
            });
        }
        else if ch.is_whitespace() {
            // do nothing
        }
        else {
            self.scan_tag_name();
        }
    }

    pub fn parse(&mut self) -> Vec<QueryToken> {
        while !self.is_end() {
            self.scan();
        }

        self.tokens.clone()
    }

}

fn tokenizer(query_string: String) -> Vec<QueryToken> {
    let mut tokens: Vec<QueryToken> = Vec::new();
    let mut tag_name: Vec<String> = Vec::new();

    for ch in query_string.chars() {
        let s = ch.to_string();

        let symbol = TokenSymbol::from_str(&s.as_str());
        let is_symbol = symbol.is_ok();

        // is a symbol and tag name non-empty, join the chars as value
        if is_symbol && !tag_name.is_empty() {
            tokens.push(QueryToken {
                value: tag_name.join(""),
                token_name: TokenSymbol::TagName,
            });
            tag_name.clear();
        }
        if is_symbol {
            tokens.push(QueryToken {
                value: s.clone(),
                token_name: symbol.unwrap()
            });
        }
        else {
            tag_name.push(s);
        }
    }

    if !tag_name.is_empty() {
        tokens.push(QueryToken {
            value: tag_name.join(""),
            token_name: TokenSymbol::TagName,
        });
    }

    return tokens;
}

fn la(query_string: String) {
    let include_tag_ids: Vec<String> = Vec::new();
    let exclude_tag_ids: Vec<String> = Vec::new();
}
