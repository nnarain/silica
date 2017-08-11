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

/// Parse commas
named!(lex_comma<&[u8], Token>,
    do_parse!(
        tag!(",") >> (Token::Comma)
    )
);

/// Parse Registers
named!(lex_registers<&[u8], Token>,
    do_parse!(
        reg: map_res!(map_res!(alt!(
            tag!("V0") |
            tag!("V1") |
            tag!("V2") |
            tag!("V3") |
            tag!("V4") |
            tag!("V5") |
            tag!("V6") |
            tag!("V7") |
            tag!("V8") |
            tag!("V9") |
            tag!("VA") |
            tag!("VB") |
            tag!("VC") |
            tag!("VD") |
            tag!("VE") |
            tag!("VF") |
            tag!("DT") |
            tag!("ST") |
            tag!("F")
        ), from_utf8), FromStr::from_str) >>
        (Token::Register(reg))
    )
);

/// Parse Directives
/// TODO: Add more directives...
named!(lex_directives<&[u8], Token>,
    do_parse!(
        directive: map_res!(map_res!(alt!(
            tag!("org") |
            tag!("todo")
        ), from_utf8), FromStr::from_str) >>
        (Token::Directive(directive))
    )
);

// Parse Instructions
named!(lex_instructions<&[u8], Token>, 
    do_parse!(
        instr: map_res!(map_res!(alt_complete!(
            tag!("CLS")  |
            tag!("RET")  |
            tag!("SYS")  |
            tag!("JP")   |
            tag!("JR")   |
            tag!("CALL") |
            tag!("SE")   |
            tag!("SNE")  |
            tag!("LD")   |
            tag!("ADD")  |
            tag!("SUBN") |
            tag!("SUB")  |
            tag!("OR")   |
            tag!("AND")  |
            tag!("XOR")  |
            tag!("SHR")  |
            tag!("SHL")  |
            tag!("RND")  |
            tag!("DRW")  |
            tag!("SKP")  |
            tag!("SKNP")
        ), from_utf8), FromStr::from_str) >>
        (Token::Instruction(instr))
    )
);

/// Consume comments
named!(lex_comments,
    do_parse!(
        tag!(";") >>
        bytes: not_line_ending >> 
        (bytes)
    )
);

/// Parse line ending
named!(lex_line_ending, 
    alt_complete!(
        tag!("\r\n") |
        tag!("\n")
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

    #[test]
    fn test_lex_comma() {
        let input = ",".as_bytes();
        let result = lex_comma(input);

        assert_eq!(result, IResult::Done(&b""[..], Token::Comma));
    }

    #[test]
    fn test_lex_registers() {
        let registers = vec!["V0", "V1", "V2", "V3", "V4", "V5", "V6", "V7", "V8", "V9", "VA", "VB", "VC", "VD", "VE", "VF", "DT", "ST", "F"];

        for register in registers.iter() {
            let result = lex_registers(register.as_bytes());
            assert_eq!(result, IResult::Done(&b""[..], Token::Register(register.to_string().clone())));
        }
    }

    #[test]
    fn test_lex_directives() {
        let directives = vec!["org", "todo"];

        for directive in directives.iter() {
            let result = lex_directives(directive.as_bytes());
            assert_eq!(result, IResult::Done(&b""[..], Token::Directive(directive.to_string().clone())));
        }
    }

    #[test]
    fn test_lex_instructions() {
        let instructions = vec!["CLS", "RET", "SYS", "JP", "CALL", "SE", "SNE", "LD", "ADD", "OR", "AND", "XOR", "SUB", "SHR", "SUBN", "SHL", "JR", "RND", "DRW", "SKP", "SKNP"];

        for instr in instructions.iter() {
            let result = lex_instructions(instr.as_bytes());
            assert_eq!(result, IResult::Done(&b""[..], Token::Instruction(instr.to_string().clone())));
        }
    }

    #[test]
    fn test_lex_comments() {
        let input = "; this is a comment\n".as_bytes();
        let result = lex_comments(input);

        assert_eq!(result, IResult::Done(&b"\n"[..], &b" this is a comment"[..]));
    }

    #[test]
    fn test_lex_line_ending_lf() {
        let input = "\n".as_bytes();
        let result = lex_line_ending(input);

        assert_eq!(result, IResult::Done(&b""[..], input));
    }

    #[test]
    fn test_lex_line_ending_crlf() {
        let input = "\r\n".as_bytes();
        let result = lex_line_ending(input);

        assert_eq!(result, IResult::Done(&b""[..], input));
    }
}