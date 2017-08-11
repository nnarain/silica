use std::fmt;
use std::str::{from_utf8, FromStr};
use std::u32;

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

/// parse a label
named!(lex_label<&[u8], Token>,
    do_parse!(
        label: map_res!(map_res!(alphanumeric, from_utf8), FromStr::from_str) >>
        (Token::Label(label))
    )
);

/// parse a hexidecimal literal
named!(lex_hex_literal<&[u8], Token>, 
    do_parse!(
        tag!("$") >>
        value: map_res!(hex_digit, from_utf8) >>
        (Token::NumericLiteral(u32::from_str_radix(value, 16).unwrap()))
    )
);

/// Parse a decimal literal
named!(lex_decimal_literal<&[u8], Token>,
    do_parse!(
        value: map_res!(digit, from_utf8) >>
        (Token::NumericLiteral(value.to_string().parse::<u32>().unwrap()))
    )
);

/// Parse either a hex or decimal literal
named!(lex_numeric_literal<&[u8], Token>,
    alt!(
        lex_decimal_literal | lex_hex_literal
    )
);

/// Parse column separator characters (spaces and tabs)
named!(lex_column_sep,
    take_while1_s!(is_space)
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
    fn test_lex_label() {
        let input = "start".as_bytes();
        let result = lex_label(input);

        assert_eq!(result, IResult::Done(&b""[..], Token::Label(String::from("start"))));
    }

    #[test]
    fn test_lex_hex_literal() {
        let input = "$A0".as_bytes();
        let result = lex_hex_literal(input);

        assert_eq!(result, IResult::Done(&b""[..], Token::NumericLiteral(0xA0)));
    }

    #[test]
    fn test_lex_decimal_literal() {
        let input = "57".as_bytes();
        let result = lex_decimal_literal(input);

        assert_eq!(result, IResult::Done(&b""[..], Token::NumericLiteral(57)));
    }

    #[test]
    fn test_lex_numeric_literal_parse_hex() {
        let input = "$FF".as_bytes();
        let result = lex_numeric_literal(input);

        assert_eq!(result, IResult::Done(&b""[..], Token::NumericLiteral(255)));
    }

    #[test]
    fn test_lex_numeric_literal_parse_decimal() {
        let input = "255".as_bytes();
        let result = lex_numeric_literal(input);

        assert_eq!(result, IResult::Done(&b""[..], Token::NumericLiteral(255)));
    }

    #[test]
    fn test_lex_column_sep_parse_all() {
        let input = " \t  \t\t".as_bytes();
        let result = lex_column_sep(input);

        assert_eq!(result, IResult::Done(&b""[..], input));
    }
    
    #[test]
    fn test_lex_column_sep_parse_until() {
        let input = " \t  \t\thello".as_bytes();
        let result = lex_column_sep(input);

        assert_eq!(result, IResult::Done(&b"hello"[..], &b" \t  \t\t"[..]));
    }
}
