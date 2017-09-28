use std::str::{from_utf8, FromStr};
use std::u32;

use nom::*;

/// Error type if lexer encounters an error in the bit stream
#[derive(Debug)]
pub struct LexerError {
    message: String
}

/// Possible tokens that can exist in the Chip8 assembly file
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Directive(String),
    Label(String),
    Instruction(String),
    Register(String),
    NumericLiteral(u32),
    LabelOperand(String),
    Comma
}

impl Token {
    pub fn is_register(&self) -> bool {
        match *self {
            Token::Register(_) => true,
            _ => false
        }
    }
    
    pub fn is_general_purpose_register(&self) -> bool {
        match *self {
            Token::Register(ref reg) =>  {
                if reg.contains("V") {
                    true
                }
                else {
                    false
                }
            },
            _ => false
        }
    }

    pub fn is_numeric_literal(&self) -> bool {
        match *self {
            Token::NumericLiteral(_) => true,
            _ => false
        }
    }
    pub fn is_label_operand(&self) -> bool {
        match *self {
            Token::LabelOperand(_) => true,
            _ => false
        }
    }
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
            tag!("F")  |
            tag!("[I]")
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
named!(lex_mnem<&[u8], Token>, 
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

/// Parse a label operand
named!(lex_label_operand<&[u8], Token>,
    do_parse!(
        tag!("#") >>
        label_operand: map_res!(map_res!(alphanumeric, from_utf8), FromStr::from_str) >>
        (Token::LabelOperand(label_operand))
    )
);

/// Parse an instruction
named!(lex_instruction<&[u8], Vec<Token>>,
    do_parse!(
        mnem: lex_mnem >>
        opt!(lex_column_sep) >>
        operand1: opt!(alt_complete!(lex_registers | lex_numeric_literal | lex_label_operand)) >>
        opt!(lex_column_sep) >>
        comma: opt!(lex_comma) >>
        opt!(lex_column_sep) >>
        operand2: opt!(alt_complete!(lex_registers | lex_numeric_literal)) >>
        ({
            let mut ret = vec![mnem];
            if let Some(operand1) = operand1 {
                ret.push(operand1);
            }
            if let Some(comma) = comma {
                ret.push(comma);
            }
            if let Some(operand2) = operand2 {
                ret.push(operand2);
            }

            ret
        })
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

/// Parse what single assembly line can end with
named!(lex_line_termination,
    do_parse!(
        opt!(lex_column_sep) >>
        opt!(lex_comments) >>
        bytes: lex_line_ending >> 
        (bytes)
    )
);

/// Parse line combination 1
/// \r\n
named!(lex_line1<&[u8], Vec<Token>>,
    do_parse!(
        lex_line_ending >>
        (Vec::new())
    )
);

/// Parse line combination 2
named!(lex_line2<&[u8], Vec<Token>>,
    do_parse!(
        lex_line_termination >>
        (Vec::new())
    )
);

/// Parse line combination 3
/// \t\t org $200
named!(lex_line3<&[u8], Vec<Token>>,
    do_parse!(
        lex_column_sep >>
        directive: lex_directives >>
        lex_column_sep >>
        numeric: lex_numeric_literal >>
        lex_line_termination >>
        (vec![directive, numeric])
    )
);

/// Parse line combination 4
/// label
named!(lex_line4<&[u8], Vec<Token>>, 
    do_parse!(
        label: lex_label >>
        lex_line_termination >>
        (vec![label])
    )
);

/// Parse line combination 5
/// LD V0, V1
named!(lex_line5<&[u8], Vec<Token>>,
    do_parse!(
        lex_column_sep >>
        instrs: lex_instruction >>
        lex_line_termination >>
        (instrs)
    )
);

/// Parse line combination 6
/// label LD V0, V1
named!(lex_line6<&[u8], Vec<Token>>,
    do_parse!(
        label: lex_label >>
        lex_column_sep >>
        instrs: lex_instruction >>
        lex_line_termination >>
        ({
            let mut tokens = vec![label];
            
            for i in instrs.iter() {
                tokens.push((*i).clone());
            }

            tokens
        })
    )
);

/// Combined line parser
named!(lex_lines<&[u8], Vec<Token>>,
    do_parse!(
        line_tokens: many0!(
            alt_complete!(
                lex_line1 |
                lex_line2 |
                lex_line3 |
                lex_line4 |
                lex_line5 |
                lex_line6
            )
        ) >>
        ({
            let mut ret = Vec::new();
            for tokens in line_tokens.iter() {
                for token in tokens.iter() {
                    ret.push((*token).clone());
                }
            }

            ret
        })
    )
);

/// Convert input bytes into tokens
pub fn tokenize(input: &[u8]) -> Result<Vec<Token>, LexerError> {
    let lexer_result = lex_lines(input);

    match lexer_result {
        IResult::Done(_, tokens) => {
            Ok(tokens)
        },
        IResult::Error(_) => {
            Err(LexerError{message: String::from("Error in lexer")})
        },
        IResult::Incomplete(_) => {
            Err(LexerError{message: String::from("Error in lexer")})
        }
    }
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
    fn test_label_operand() {
        let input = "#label".as_bytes();
        let result = lex_label_operand(input);

        assert_eq!(result, IResult::Done(&b""[..], Token::LabelOperand(String::from("label"))));
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
    fn test_lex_mnem() {
        let instructions = vec!["CLS", "RET", "SYS", "JP", "CALL", "SE", "SNE", "LD", "ADD", "OR", "AND", "XOR", "SUB", "SHR", "SUBN", "SHL", "JR", "RND", "DRW", "SKP", "SKNP"];

        for instr in instructions.iter() {
            let result = lex_mnem(instr.as_bytes());
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

    #[test]
    fn test_lex_line_termination1() {
        let input = " ; comment\r\n".as_bytes();
        let result = lex_line_termination(input);

        assert_eq!(result, IResult::Done(&b""[..], &b"\r\n"[..]));
    }

    #[test]
    fn test_lex_line_termination2() {
        let input = "; comment\r\n".as_bytes();
        let result = lex_line_termination(input);

        assert_eq!(result, IResult::Done(&b""[..], &b"\r\n"[..]));
    }

    #[test]
    fn test_lex_line_termination3() {
        let input = "\r\n".as_bytes();
        let result = lex_line_termination(input);

        assert_eq!(result, IResult::Done(&b""[..], &b"\r\n"[..]));
    }

    #[test]
    fn test_lex_instruction1() {
        let input = "RET\n".as_bytes();
        let result = lex_instruction(input);

        let expected_tokens = vec![Token::Instruction(String::from("RET"))];

        assert_eq!(result, IResult::Done(&b"\n"[..], expected_tokens));
    }
    
    #[test]
    fn test_lex_instruction2() {
        let input = "JP $200\n".as_bytes();
        let result = lex_instruction(input);

        let expected_tokens = vec![Token::Instruction(String::from("JP")), Token::NumericLiteral(0x200)];

        assert_eq!(result, IResult::Done(&b"\n"[..], expected_tokens));
    }

    #[test]
    fn test_lex_instruction3() {
        let input = "LD V0, V1\n".as_bytes();
        let result = lex_instruction(input);

        let expected_tokens = vec![
            Token::Instruction(String::from("LD")),
            Token::Register(String::from("V0")),
            Token::Comma,
            Token::Register(String::from("V1"))
        ];

        assert_eq!(result, IResult::Done(&b"\n"[..], expected_tokens));
    }

    #[test]
    fn test_lex_instruction4() {
        let input = "LD V0, $FF\n".as_bytes();
        let result = lex_instruction(input);

        let expected_tokens = vec![
            Token::Instruction(String::from("LD")),
            Token::Register(String::from("V0")),
            Token::Comma,
            Token::NumericLiteral(0xFF)
        ];

        assert_eq!(result, IResult::Done(&b"\n"[..], expected_tokens));
    }

    #[test]
    fn test_lex_instruction5() {
        let input = "JP #label\n".as_bytes();
        let result = lex_instruction(input);

        let expected_tokens = vec![
            Token::Instruction(String::from("JP")),
            Token::LabelOperand(String::from("label"))
        ];

        assert_eq!(result, IResult::Done(&b"\n"[..], expected_tokens));
    }

    #[test]
    fn test_lex_line1_lf() {
        let input = "\n".as_bytes();
        let result = lex_line1(input);

        assert_eq!(result, IResult::Done(&b""[..], Vec::new()));
    }

    #[test]
    fn test_lex_line1_crlf() {
        let input = "\r\n".as_bytes();
        let result = lex_line1(input);

        assert_eq!(result, IResult::Done(&b""[..], Vec::new()));
    }

    #[test]
    fn test_lex_line2() {
        let input = "\t\t\t\t  org $200\n".as_bytes();
        let result = lex_line2(input);

        let expected_directive = Token::Directive(String::from("org"));
        let expected_numeric = Token::NumericLiteral(0x200u32);

        assert_eq!(result, IResult::Done(&b""[..], vec![expected_directive, expected_numeric]));
    }

    #[test]
    fn test_lex_line3() {
        let input = "label\n".as_bytes();
        let result = lex_line3(input);

        let expected_directive = Token::Label(String::from("label"));

        assert_eq!(result, IResult::Done(&b""[..], vec![expected_directive]));
    }

    #[test]
    fn test_lex_line4() {
        let input = "\t\t LD V0, V1\n".as_bytes();
        let result = lex_line4(input);

        let expected_tokens = vec![
            Token::Instruction(String::from("LD")),
            Token::Register(String::from("V0")),
            Token::Comma,
            Token::Register(String::from("V1"))
        ];

        assert_eq!(result, IResult::Done(&b""[..], expected_tokens));
    }

    #[test]
    fn test_lex_line5() {
        let input = "label\t\t LD V0, V1\n".as_bytes();
        let result = lex_line5(input);

        let expected_tokens = vec![
            Token::Label(String::from("label")),
            Token::Instruction(String::from("LD")),
            Token::Register(String::from("V0")),
            Token::Comma,
            Token::Register(String::from("V1"))
        ];

        assert_eq!(result, IResult::Done(&b""[..], expected_tokens));
    }

    #[test]
    fn test_lex_lines1() {
        let input = "label\t\t LD V0, V1\n".as_bytes();
        let result = lex_lines(input);

        let expected_tokens = vec![
            Token::Label(String::from("label")),
            Token::Instruction(String::from("LD")),
            Token::Register(String::from("V0")),
            Token::Comma,
            Token::Register(String::from("V1"))
        ];

        assert_eq!(result, IResult::Done(&b""[..], expected_tokens));
    }

    #[test]
    fn test_lex_lines2() {
        let input = "\t\t LD V0, V1\n".as_bytes();
        let result = lex_lines(input);

        let expected_tokens = vec![
            Token::Instruction(String::from("LD")),
            Token::Register(String::from("V0")),
            Token::Comma,
            Token::Register(String::from("V1"))
        ];

        assert_eq!(result, IResult::Done(&b""[..], expected_tokens));
    }

    #[test]
    fn test_lex_lines3() {
        let input = "\t\t org $200\n".as_bytes();
        let result = lex_lines(input);

        let expected_tokens = vec![
            Token::Directive(String::from("org")),
            Token::NumericLiteral(0x200)
        ];

        assert_eq!(result, IResult::Done(&b""[..], expected_tokens));
    }

    #[test]
    fn test_lex_blank_line() {
        let input = "label1\t\tLD V0, $FF ; comment 1\n\nend\t\tJP #end ; comment 2\n".as_bytes();
        let result = lex_lines(input);

        let expected_tokens = vec![
            Token::Label(String::from("label1")),
            Token::Instruction(String::from("LD")), Token::Register(String::from("V0")), Token::Comma, Token::NumericLiteral(0xFF),
            Token::Label(String::from("end")),
            Token::Instruction(String::from("JP")), Token::LabelOperand(String::from("end"))
        ];

        assert_eq!(result, IResult::Done(&b""[..], expected_tokens));
    }
}