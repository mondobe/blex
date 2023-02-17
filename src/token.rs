use std::ops::Range;
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub struct Token<'a> {
    pub body: &'a str,
    pub indices: Range<usize>,
    pub tags: Vec<&'a str>
}

impl Default for Token<'_> {
    fn default() -> Self {
        empty_token()
    }
}

impl <'a>Token<'a> {
    pub fn content(&'a self) -> &'a str {
        &self.body[self.indices.clone()]
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&tag)
    }

    pub fn single_char(&self) -> Option<char> {
        self.content().chars().next()
    }
}

pub fn empty_token() -> Token<'static> {
    Token {
        body: "",
        indices: 0..0,
        tags: vec![]
    }
}

impl <'a>fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: ", self.content()).unwrap();
        for tag in &self.tags {
            write!(f, "{}; ", tag).unwrap();
        };
        Ok(())
    }
}

pub fn token_from_string<'a>(content: &'a str, tags: Vec<&'a str>) -> Token<'a> {
    Token {
        body: content,
        indices: 0..content.len(),
        tags
    }
}

pub fn print_tokens(tokens: Vec<Token>) {
    for tok in tokens {
        println!("{}", tok);
    }
}

pub fn str_to_tokens<'a>(body: &'a str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    for index in 0..body.len() {
        tokens.push(Token {
            body,
            indices: index..index + 1,
            tags: vec![&body[index..index + 1]]
        });
    }
    tokens.push(empty_token());
    tokens
}

pub fn wrap<'a>(tokens: Vec<Token<'a>>, tags: Vec<&'a str>) -> Token<'a> {
    if tokens.is_empty() {
        return empty_token();
    }
    Token { 
        body: tokens[0].body, 
        indices: tokens[0].indices.start..tokens.last().unwrap().indices.end, 
        tags
    }
}

pub enum TokenStructure<'a> {
    Multiple,
    Single(&'a Token<'a>)
}

pub fn tokens_structure<'a>(tokens: &'a Vec<Token<'a>>) -> TokenStructure<'a> {
    if tokens.len() == 1 {
        TokenStructure::Single(&tokens[0])
    } else {
        TokenStructure::Multiple
    }
}