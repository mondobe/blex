pub mod token;
pub use token::*;

/// Processes a rule across a vector of tokens. Starting from the first token,
/// iteratively applies the rule on a single token. If that application returns
/// None, continues applying the rule on the token and the next, then the next,
/// and so on until the function returns Some or there are no tokens left to
/// process. This process is explained more thoroughly with examples in the 
/// readme.
pub fn process_rule(rule: impl Fn(Vec<Token>) -> Option<Vec<Token>>, body: &mut Vec<Token>) {
    // iterate through each starting position in the body
    let mut start_index: usize = 0;
    'current_start: while start_index < body.len() {
        // the end_index will increment if the rule returns None
        let mut end_index: usize = start_index + 1;
        let mut tokens: Vec<Token> = body[start_index..end_index].to_vec();

        // apply the rule to the tokens
        let mut applied: Option<Vec<Token>> = rule(tokens);

        // if the rule returns Some or requests tokens past the end, finish the iteration
        while let None = applied {
            end_index += 1;
            if end_index > body.len() {
                start_index += 1;
                continue 'current_start;
            }
            tokens = body[start_index..end_index].to_vec();
            applied = rule(tokens);
        }

        // we know that the returned tokens will exist at this point, so unwrap() is safe
        let replacement: Vec<Token> = applied.unwrap();

        println!("\nReplacing");
        print_tokens(body[start_index..end_index].to_vec());
        println!("with");
        print_tokens(replacement.clone());

        let r_len = replacement.len();

        // insert the new tokens where the old ones were
        body.splice(start_index..end_index, replacement);
        if r_len >= end_index - start_index {
            start_index += 1;
        }
    }
}

/// Processes multiple rules on a vector of tokens. See [process_rule].
pub fn process_rules(rules: Vec<impl Fn(Vec<Token>) -> Option<Vec<Token>>>, body: &mut Vec<Token>) {
    for rule in rules {
        process_rule(rule, body);
    }
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    fn token1() -> Token<'static> {
        token_from_string("Hi!", vec!["greeting", "exclam"])
    }

    fn token2() -> Token<'static> {
        token_from_string("-890", vec!["int", "neg_int"])
    }

    fn test_tokens() -> Vec<Token<'static>> {
        vec![token1(), token2()]
    }

    #[test]
    fn print_one_token() {
        println!("{}", token1());
    }

    #[test]
    fn print_several_tokens() {
        print_tokens(test_tokens());
    }

    #[test]
    fn print_base_token_stream() {
        print_tokens(str_to_tokens("This is a string."));
    }

    fn whitespace_rule(mut tokens: Vec<Token>) -> Option<Vec<Token>> {
        let ch = tokens[0].single_char().unwrap_or_default();
        if ch.is_whitespace() || ch == '\u{0}' {
            tokens[0].tags.push("ws");
        }
        Some(tokens)
    }

    fn letter_rule(mut tokens: Vec<Token>) -> Option<Vec<Token>> {
        if tokens[0].single_char().unwrap_or_default().is_alphabetic() {
            tokens[0].tags.push("letter");
        }
        Some(tokens)
    }

    fn word_rule(tokens: Vec<Token>) -> Option<Vec<Token>> {
        if tokens.last().unwrap_or(&empty_token()).has_tag("letter") {
            None
        } else if tokens.len() == 1 {
            Some(tokens)
        } else {
            Some(vec![
                wrap(tokens[0..tokens.len() - 1].to_vec(), vec!["word"]),
                tokens.last().unwrap().clone(),
            ])
        }
    }

    fn int_rule(tokens: Vec<Token>) -> Option<Vec<Token>> {
        match tokens_structure(&tokens) {
            TokenStructure::Single(tok) => {
                if tok.content() == "0" {
                    Some(vec![wrap(tokens, vec!["int", "posInt"])])
                } else if tok.has_tag("digit") {
                    None
                } else {
                    Some(tokens)
                }
            }
            TokenStructure::Multiple => {
                if tokens.last().unwrap_or(&empty_token()).has_tag("digit") {
                    None
                } else {
                    Some(vec![
                        wrap(tokens[0..tokens.len() - 1].to_vec(), vec!["int", "posInt"]),
                        tokens.last().unwrap().clone(),
                    ])
                }
            }
            TokenStructure::None => {
                Some(tokens)
            }
        }
    }

    fn remove_whitespace_rule(tokens: Vec<Token>) -> Option<Vec<Token>> {
        if tokens[0].has_tag("ws") {
            Some(vec![])
        } else {
            Some(tokens)
        }
    }

    fn digit_rule(mut tokens: Vec<Token>) -> Option<Vec<Token>> {
        if let TokenStructure::Single(tok) = tokens_structure(&tokens) {
            let ch = tok.single_char().unwrap_or_default();
            if ch.is_digit(10) {
                tokens[0].tags.push("digit");
                if ch != '0' {
                    tokens[0].tags.push("nonzero");
                }
            }
        }
        Some(tokens)
    }

    #[test]
    fn apply_one_rule() {
        let mut body = str_to_tokens("A space");
        print_tokens(body.clone());
        process_rule(whitespace_rule, &mut body);
        print_tokens(body);
    }

    fn word_rules() -> Vec<impl Fn(Vec<Token>) -> Option<Vec<Token>>> {
        [
            whitespace_rule,
            letter_rule,
            word_rule,
            remove_whitespace_rule,
        ]
        .to_vec()
    }

    fn int_rules() -> Vec<impl Fn(Vec<Token>) -> Option<Vec<Token>>> {
        [
            whitespace_rule,
            digit_rule,
            int_rule,
            remove_whitespace_rule,
        ]
        .to_vec()
    }

    fn ab_rule(tokens: Vec<Token>) -> Option<Vec<Token>> {
        match tokens_structure(&tokens) {
            TokenStructure::Single(tok) => {
                if tok.has_tag("a") {
                    None
                } else {
                    Some(tokens)
                }
            }
            TokenStructure::Multiple => {
                if tokens[1].has_tag("b") {
                    Some(vec![wrap(tokens, vec!["c"])])
                } else {
                    Some(tokens)
                }
            }
            TokenStructure::None => {
                Some(tokens)
            }
        }
    }

    #[test]
    fn apply_ab() {
        let text = "a b blex ab abab";
        let mut body = str_to_tokens(text);
        process_rule(ab_rule, &mut body);
        print_tokens(body);
    }

    #[test]
    fn apply_words() {
        let text = "
(define (rgb-series mk)
  (vc-append
   (series (lambda (sz) (colorize (mk sz) \"red\")))
   (series (lambda (sz) (colorize (mk sz) \"green\")))
   (series (lambda (sz) (colorize (mk sz) \"blue\")))))";
        let mut body = str_to_tokens(text);
        process_rules(word_rules(), &mut body);
        print_tokens(body.clone());
        assert_eq!(
            body,
            vec![
                Token {
                    body: text,
                    indices: 0..1,
                    tags: vec!["word"]
                },
                Token {
                    body: text,
                    indices: 5..10,
                    tags: vec!["word"]
                }
            ]
        );
    }

    #[test]
    fn big_paragraph_performance() {
        let text = "The donkey than rams him.
        Oh my goodness, but the kangaroo jumps over.
        And it looks like the seagulls are going for it again!
        They're just hitting the tank!
        (To the penguins, attacking a T-Rex)
        Hit him with your penguin beaks!
        What are you doing out there?
        Looks like I gotta do everything myself...
        Come on, now I'm playing.
        Get over here, T-Rex. I'll beat you up.
        Now watch out for my spin attack...";
        let mut body = str_to_tokens(text);
        for _ in 0..1000 {
            black_box(process_rules(word_rules(), &mut body));
        }
    }

    #[test]
    fn apply_ints() {
        let text = "123 040 k";
        let mut body = str_to_tokens(text);
        process_rules(int_rules(), &mut body);
        assert_eq!(
            body,
            vec![
                Token {
                    body: text,
                    indices: 0..3,
                    tags: vec!["int", "posInt"]
                },
                Token {
                    body: text,
                    indices: 4..5,
                    tags: vec!["int", "posInt"]
                },
                Token {
                    body: text,
                    indices: 5..7,
                    tags: vec!["int", "posInt"]
                },
                Token {
                    body: text,
                    indices: 8..9,
                    tags: vec!["k"]
                }
            ]
        );
    }

    #[test]
    fn has_tag_test() {
        println!("{}", token_from_string("Hi", vec!["test"]).has_tag("test"));
        print_tokens(str_to_tokens("a b blex ab abab"));
    }
}
