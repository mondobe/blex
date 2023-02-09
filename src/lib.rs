pub mod token;
pub use token::*;

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

        // insert the new tokens where the old ones were
        body.splice(start_index..end_index, replacement);
        start_index += 1;
    }
}

pub fn process_rules(rules: Vec<impl Fn(Vec<Token>) -> Option<Vec<Token>>>, body: &mut Vec<Token>) {
    for rule in rules {
        process_rule(rule, body);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token1() -> Token<'static> {
        token_from_string(
            "Hi!",
            vec!["greeting", "exclam"]
        )
    }

    fn token2() -> Token<'static> {
        token_from_string(
            "-890",
            vec!["int", "neg_int"]
        )
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
        print_tokens(
            str_to_tokens("This is a string.")
        );
    }

    fn whitespace_rule(mut tokens: Vec<Token>) -> Option<Vec<Token>> {
        if tokens[0].content().is_empty() {
            tokens[0].tags.push("ws");
        } else if tokens[0].content().chars().collect::<Vec<char>>()[0].is_whitespace() {
            tokens[0].tags.push("ws");
        } 
        Some(vec![tokens[0].clone()])
    }

    fn letter_rule(mut tokens: Vec<Token>) -> Option<Vec<Token>> {
        if tokens[0].content().is_empty() {
            return Some(vec![tokens[0].clone()]);
        }
        if tokens[0].content().chars().collect::<Vec<char>>()[0].is_alphabetic() {
            tokens[0].tags.push("letter");
        }
        Some(vec![tokens[0].clone()])
    }

    fn word_rule(tokens: Vec<Token>) -> Option<Vec<Token>> {
        if tokens.last().unwrap().has_tag("letter") {
            None
        } else if tokens.len() == 1 {
            Some(tokens)
        } else {
            Some(vec![
                wrap(tokens[0..tokens.len() - 1].to_vec(), vec!["word"]),
                tokens.last().unwrap().clone()
            ])
        }
    }

    fn remove_whitespace_rule(tokens: Vec<Token>) -> Option<Vec<Token>> {
        if tokens[0].has_tag("ws") {
            Some(vec![])
        } else {
            Some(tokens)
        }
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
            remove_whitespace_rule
        ].to_vec()
    }

    #[test]
    fn apply_words() {
        let text = "A space";
        let mut body = str_to_tokens(text);
        process_rules(word_rules(), &mut body);
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
                    indices: 2..7,
                    tags: vec!["word"]
                }
            ]
        );
    }

    #[test]
    fn has_tag_test() {
        println!("{}", token_from_string("Hi", vec!["test"]).has_tag("test"));
    }
}
