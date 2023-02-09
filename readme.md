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