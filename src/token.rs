use std::ops::Range;
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub struct Token<'a> {
    pub body: &'a str,
    pub indices: Range<usize>,
    pub tags: Vec<&'a str>
}

impl <'a>Token<'a> {
    pub fn content(&'a self) -> &'a str {
        &self.body[self.indices.clone()]
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&tag)
    }
}

impl <'a>fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: ", self.content()).unwrap();
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
    tokens.push(Token {
        body: "",
        indices: 0..0,
        tags: vec![]
    });
    tokens
}

pub fn wrap<'a>(tokens: Vec<Token<'a>>, tags: Vec<&'a str>) -> Token<'a> {
    if tokens.is_empty() {
        return Token {
            body: "",
            indices: 0..0,
            tags
        };
    }
    Token { 
        body: tokens[0].body, 
        indices: tokens[0].indices.start..tokens.last().unwrap().indices.end, 
        tags
    }
}