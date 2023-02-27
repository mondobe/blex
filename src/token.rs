use std::fmt;
use std::ops::Range;

#[derive(Clone, PartialEq, Debug)]

/// A representation of a token. The `body` field points to the underlying string
/// from which the characters in this token come. The `indices` field contains
/// a range representing which contiguous characters are used. The `tags` field is
/// a vector of borrowed strings with user-defined values.
pub struct Token<'a> {
    pub body: &'a str,
    pub indices: Range<usize>,
    pub tags: Vec<&'a str>,
}

impl Default for Token<'_> {
    /// A token's default value is the empty token, which borrows 0 characters
    /// from a new empty string.
    fn default() -> Self {
        empty_token()
    }
}

impl<'a> Token<'a> {
    /// Gets the content of the token as a string slice from the body. Mostly used
    /// for printing tokens.
    pub fn content(&'a self) -> &'a str {
        &self.body[self.indices.clone()]
    }

    /// Whether or not a token's tags contain a certain string slice value.
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&tag)
    }

    /// If this token is a single character long, returns that character. Otherwise,
    /// returns None.
    pub fn single_char(&self) -> Option<char> {
        self.content().chars().next()
    }
}

/// A token with no contents and a static body.
pub fn empty_token() -> Token<'static> {
    Token {
        body: "",
        indices: 0..0,
        tags: vec![],
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: ", self.content()).unwrap();
        for tag in &self.tags {
            write!(f, "{}; ", tag).unwrap();
        }
        Ok(())
    }
}

/// Consolidates a single string slice and a vector of tags into one token.
/// Mostly used for debugging.
pub fn token_from_string<'a>(content: &'a str, tags: Vec<&'a str>) -> Token<'a> {
    Token {
        body: content,
        indices: 0..content.len(),
        tags,
    }
}

/// Prints a vector of tokens using their default Display method.
pub fn print_tokens(tokens: Vec<Token>) {
    for tok in tokens {
        println!("{}", tok);
    }
}

/// Chops up a string slice into a vector of owned tokens. Also appends an empty
/// token to the tail of the vector to enable certain lexing functions like
/// scanning for words.
pub fn str_to_tokens<'a>(body: &'a str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    for index in 0..body.len() {
        tokens.push(Token {
            body,
            indices: index..index + 1,
            tags: vec![&body[index..index + 1]],
        });
    }
    tokens.push(empty_token());
    tokens
}

/// Wraps up a vector of consecutive tokens into one token and applies the
/// specified tags.
pub fn wrap<'a>(tokens: Vec<Token<'a>>, tags: Vec<&'a str>) -> Token<'a> {
    if tokens.is_empty() {
        return empty_token();
    }
    Token {
        body: tokens[0].body,
        indices: tokens[0].indices.start..tokens.last().unwrap().indices.end,
        tags,
    }
}

/// Represents the possible shapes of a vector of tokens. A vector of tokens could
/// contain multiple tokens, a single token (a common special case), or none at
/// all (a rare corner case).
pub enum TokenStructure<'a> {
    Multiple,
    Single(&'a Token<'a>),
    None,
}

/// Goes with the TokenStructure enum. Specifies whether a borrowed vector of
/// tokens consists of multiple tokens, a single token, or none at all.
pub fn tokens_structure<'a>(tokens: &'a Vec<Token<'a>>) -> TokenStructure<'a> {
    if tokens.len() == 1 {
        TokenStructure::Single(&tokens[0])
    } else if tokens.len() > 1 {
        TokenStructure::Multiple
    } else {
        TokenStructure::None
    }
}
