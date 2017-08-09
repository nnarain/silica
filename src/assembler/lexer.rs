use std::fmt;
use std::str::{from_utf8, FromStr};

use nom::*;

/// Error type if lexer encounters an error in the bit stream
pub struct TokenError {
    message: String
}

impl fmt::Debug for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Possible tokens that can exist in the Chip8 assembly file
#[derive(Debug, PartialEq)]
pub enum Token {
    Directive(String),
    Label(String),
    Instruction(String),
    Register(String),
    NumericLiteral(u32),
    Comma
}

named!(lex_label<&[u8], Token>,
    do_parse!(
        label: map_res!(map_res!(alphanumeric, from_utf8), FromStr::from_str) >>
        (Token::Label(label))
    )
);

/// Convert input bytes into tokens
pub fn tokenize(input: &[u8]) -> Result<Vec<Token>, TokenError> {
    let tokens: Vec<Token> = Vec::new();

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label() {
        let input = "start".as_bytes();
        let result = lex_label(input);

        assert_eq!(result, IResult::Done(&b""[..], Token::Label(String::from("start"))));
    }
}
