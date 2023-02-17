Blex is a lightweight lexing framework built in Rust based on the T-Lex framework. Blex is built around the concept of building a set of rules that process a set of tokens. Each rule is run throughout the input string successively, gradually transforming it into an output string of tokens.

## Tokens

The implementation of tokens itself is the idea most heavily borrowed from Rustuck. Tokens in Blex are represented as a contiguous range of characters from the original string along with a set of tags, themselves also represented as borrowed strings. For example, the string `let x: int = 2;` might be represented as:

```
"let": var_dec, keyword
"x": ident
":": colon, keyword
"int": ident
"=": eq, keyword
"2": int
```

Notice the lack of whitespace: keywords don't have to represent the entire string.

## Rules

Rules in Blex are any function that transforms a vector of tokens into an optional vector of tokens. In Rust, that refers to this trait bound: `Fn(Vec<Token>) -> Option<Vec<Token>>`. Rules may modify the input tokens without worrying about mutating the original string: tokens are cloned before the rules are applied to them (luckily, cloning is a very cheap operation for tokens, which are just a range and a few pointers). 

Rules are processed with the `process_rule` and `process_rules` function. The `process_rule` function applies the rule starting on each token in a list. Rules are applied starting with a single token, which is wrapped in a `Vec`, passed into the function, and processed. If the function returns `None`, the starting token and the next token are combined into a `Vec` and passed into the function. If the function returns `Some`, the tokens passed in are replaced with the returned `Vec`. 

## Rule Processing

Rules are processed on strings of tokens, but strings of characters are transformed into tokens using the `str_to_tokens` function. Each character of a string is turned into a corresponding token, and one tag is added with the content of the character, a holdover from Rustuck. 

### An Example

Let's create a rule that takes two consecutive tokens that read `ab` and converts them into one token with the tag `c`. The rule would start out like this:

```
fn ab_rule(tokens: Vec<Token>) -> Option<Vec<Token>> {

}
```

Let's say we input the string `a b blex ab abab`. The string will be turned into these tokens:

```
"a": a; 
" ":  ; 
"b": b; 
" ":  ; 
"b": b; 
"l": l; 
"e": e; 
"x": x;
" ":  ;
"a": a;
"b": b;
" ":  ;
"a": a;
"b": b;
"a": a;
"b": b;
"":
```

Notice the empty token at the end. We didn't type that: it was added automatically to give some rules, like those testing for words, a buffer. 
Our rule will start by scanning each token individually. Remember that we are scanning for the pattern "ab". Let's chart out each possible route our rule could take.

- It will start by finding one token containing one character.
	- If the token has the tag `a`, we should continue the rule to include the next token.
	- Otherwise, we should stop the rule and return the token unchanged.
- If multiple tokens are found, based on the last rule, it must be a pair of tokens with the first one being `a`. Knowing that, there are two paths we can take:
	- If the second token has the tag `b`, we should combine those tokens and give it the tag `c`.
	- Otherwise, we should stop the rule and return the token unchanged.

Luckily, blex has idiomatic ways to express this. We'll start by `match`ing on `token_structure(&tokens)`, which has two options:

```
fn ab_rule(tokens: Vec<Token>) -> Option<Vec<Token>> {
	match tokens_structure(&tokens) {
            TokenStructure::Single(tok) => {
	            
            },
            TokenStructure::Multiple => {
				
            }
        }
}
```

The `token_structure` function takes in a borrowed `Vec` of tokens. If it finds that the `Vec` holds only one token, it returns that token wrapped in a `Single`. Otherwise, it returns `Multiple`. This is a safe way to guarantee the existence of one token.
There is also an idiomatic way to test a token for certain tags in the `has_tag` method. We can use this to test both cases for the `a` and `b` tags respectively.

```
fn ab_rule(tokens: Vec<Token>) -> Option<Vec<Token>> {
	match tokens_structure(&tokens) {
            TokenStructure::Single(tok) => {
	            if tok.has_tag("a") {
		            // case 1
                } else {
					// case 2
                }
            },
            TokenStructure::Multiple => {
				if tokens[1].has_tag("b") {
					// case 3
                } else {
					// case 4
                }
            }
        }
}
```

The only thing left is the return values: 
- In case 1, we want to continue the rule to the next token. We do this by returning `None`.
- In cases 2 and 4, we want to end the rule and return the tokens unchanged. We simply do this by returning `Some(tokens)`.
- In case 3, we want to combine both of our tokens and give them the tag `c`. Of course, there is an idiomatic way to do this as well: `wrap`. The `wrap` function takes a `Vec` of tokens (which are assumed to be from the same string of tokens) and combines all of their contents into one token. The second argument of `wrap` is a `Vec<&str>`, which contains all of the tags to add to the new token.

```
fn ab_rule(tokens: Vec<Token>) -> Option<Vec<Token>> {
	match tokens_structure(&tokens) {
		TokenStructure::Single(tok) => {
			if tok.has_tag("a") {
				None
			} else {
				Some(tokens)
			}
		},
		TokenStructure::Multiple => {
			if tokens[1].has_tag("b") {
				Some(vec![wrap(tokens, vec!["c"])])
			} else {
				Some(tokens)
			}
		}
	}
}
```

That completes our rule! We can test it with a script like this:

```
#[test]
fn apply_ab() {
	let text = "a b blex ab abab";
	let mut body = str_to_tokens(text);
	process_rule(ab_rule, &mut body);
	print_tokens(body);
}
```

Which gives us:

```
"a": a; 
" ":  ; 
"b": b; 
" ":  ; 
"b": b; 
"l": l; 
"e": e;
"x": x;
" ":  ;
"ab": c;
" ":  ;
"ab": c;
"ab": c;
"":
```