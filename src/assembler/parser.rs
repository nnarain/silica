use assembler::lexer::*;
use nom::*;

/// an expression is a certain combination of tokens
pub type Expression = Vec<Token>;

/// parser errors
#[derive(Debug)]
pub struct ParserError {
    message: String
}

macro_rules! tag_token {
    ($i: expr, $tag: pat) => (
        {
            let ret: IResult<&[Token], Token, u32> = 
            if $i.len() > 0 {
                match $i[0] {
                    $tag => {
                        IResult::Done(&$i[1..], $i[0].clone())
                    },
                    _ => {
                        IResult::Error(error_position!(ErrorKind::Tag, $i))
                    }
                }
            }
            else {
                IResult::Incomplete(Needed::Size(1))
            };

            ret
        }
    )
}

macro_rules! opt_complete(
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
    {
        let i_ = $i.clone();
        match $submac!(i_, $($args)*) {
            IResult::Done(i,o)     => IResult::Done(i, ::std::option::Option::Some(o)),
            _                      => {
                let res: IResult<_,_> = IResult::Done($i, ::std::option::Option::None);
                res
            },
        }
    }
    );
    ($i:expr, $f:expr) => (
        opt_complete!($i, call!($f));
    );
);

/// parse labels from tokens
named!(parse_label<&[Token], Expression>,
    do_parse!(
        label: tag_token!(Token::Label(_)) >>
        (vec![label])
    )    
);

/// parse directive
named!(parse_directive<&[Token], Expression>,
    do_parse!(
        directive: tag_token!(Token::Directive(_)) >>
        num: tag_token!(Token::NumericLiteral(_)) >>
        (vec![directive, num])
    )
);

/// parse instructions
named!(parse_instructions<&[Token], Expression>,
    do_parse!(
        instr: tag_token!(Token::Instruction(_)) >>
        operand1: opt_complete!(alt_complete!(
            tag_token!(Token::Register(_)) |
            tag_token!(Token::NumericLiteral(_)) |
            tag_token!(Token::LabelOperand(_))
        )) >>
        opt_complete!(tag_token!(Token::Comma)) >>
        operand2: opt_complete!(alt_complete!(
            tag_token!(Token::Register(_)) |
            tag_token!(Token::NumericLiteral(_))
        )) >>
        ({
            let mut ret = vec![instr];
            if let Some(operand1) = operand1 {
                ret.push(operand1);
            }
            if let Some(operand2) = operand2 {
                ret.push(operand2);
            }

            ret
        })
    )
);

/// parse expressions from the token stream
named!(parse_expressions<&[Token], Vec<Expression>>,
    do_parse!(
        exprs: many0!(
            alt_complete!(
                parse_directive |
                parse_label |
                parse_instructions
            )
        ) >>
        (exprs)
    )
);

/// parse expressions from tokens
pub fn parse(tokens: Vec<Token>) -> Result<Vec<Expression>, ParserError> {
    let result = parse_expressions(&tokens[..]);

    match result {
        IResult::Done(_, exprs) => Ok(exprs),
        _ => Err(ParserError{message: String::from("Error parsing tokens")})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_label() {
        let input = vec![Token::Label(String::from("JP"))];
        let result = parse_label(&input[..]);
        let empty: Vec<Token> = vec![];

        assert_eq!(result, IResult::Done(&empty[..], vec![Token::Label(String::from("JP"))]));
    }

    #[test]
    fn test_parse_directive() {
        let input = vec![Token::Directive(String::from("org")), Token::NumericLiteral(0x200)];
        let result = parse_directive(&input[..]);
        let empty: Vec<Token> = vec![];

        assert_eq!(result, IResult::Done(&empty[..], vec![Token::Directive(String::from("org")), Token::NumericLiteral(0x200)]));
    }

    #[test]
    fn test_parse_instruction1() {
        let input = vec![
            Token::Instruction(String::from("CLS"))
        ];
        let result = parse_instructions(&input[..]);
        let empty: Vec<Token> = vec![];

        assert_eq!(result, IResult::Done(&empty[..], vec![Token::Instruction(String::from("CLS"))]));
    }

    #[test]
    fn test_parse_instruction2() {
        let input = vec![
            Token::Instruction(String::from("JP")),
            Token::NumericLiteral(0x200)
        ];
        let result = parse_instructions(&input[..]);
        let empty: Vec<Token> = vec![];

        assert_eq!(result, IResult::Done(&empty[..], vec![
            Token::Instruction(String::from("JP")),
            Token::NumericLiteral(0x200)
        ]));
    } 

    #[test]
    fn test_parse_instruction3() {
        let input = vec![
            Token::Instruction(String::from("LD")),
            Token::Register(String::from("V0")),
            Token::Comma,
            Token::Register(String::from("V1"))
        ];
        let result = parse_instructions(&input[..]);
        let empty: Vec<Token> = vec![];

        assert_eq!(result, IResult::Done(&empty[..], vec![
            Token::Instruction(String::from("LD")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("V1"))
        ]));
    }

    #[test]
    fn test_parse_instruction4() {
        let input = vec![
            Token::Instruction(String::from("LD")),
            Token::Register(String::from("V0")),
            Token::Comma,
            Token::NumericLiteral(5)
        ];
        let result = parse_instructions(&input[..]);
        let empty: Vec<Token> = vec![];

        assert_eq!(result, IResult::Done(&empty[..], vec![
            Token::Instruction(String::from("LD")),
            Token::Register(String::from("V0")),
            Token::NumericLiteral(5)
        ]));
    }

    #[test]
    fn test_parse_instruction5() {
        let input = vec![
            Token::Instruction(String::from("JP")),
            Token::LabelOperand(String::from("end"))
        ];
        let result = parse_instructions(&input[..]);
        let empty: Vec<Token> = vec![];

        assert_eq!(result, IResult::Done(&empty[..], vec![
            Token::Instruction(String::from("JP")),
            Token::LabelOperand(String::from("end"))
        ]));
    }

    #[test]
    fn test_parse1() {
        let input = vec![
            Token::Directive(String::from("org")), Token::NumericLiteral(0x200),
            Token::Label(String::from("entry")),
            Token::Instruction(String::from("CLS"))
        ];
        let result = parse(input).unwrap();

        assert_eq!(result, vec![
            vec![Token::Directive(String::from("org")), Token::NumericLiteral(0x200)],
            vec![Token::Label(String::from("entry"))],
            vec![Token::Instruction(String::from("CLS"))]
        ]);
    }

    #[test]
    fn test_parse2() {
        let input = vec![
            Token::Label(String::from("label1")),
            Token::Instruction(String::from("LD")), Token::Register(String::from("V1")), Token::NumericLiteral(0xFF),
            Token::Label(String::from("label2")),
            Token::Instruction(String::from("LD")), Token::Register(String::from("V0")), Token::NumericLiteral(0x05),
            Token::Label(String::from("end")),
            Token::Instruction(String::from("JP")), Token::Register(String::from("#end"))
        ];
        let result = parse(input).unwrap();

        assert_eq!(result, vec![
            vec![Token::Label(String::from("label1"))],
            vec![Token::Instruction(String::from("LD")), Token::Register(String::from("V1")), Token::NumericLiteral(0xFF)],
            vec![Token::Label(String::from("label2"))],
            vec![Token::Instruction(String::from("LD")), Token::Register(String::from("V0")), Token::NumericLiteral(0x05)],
            vec![Token::Label(String::from("end"))],
            vec![Token::Instruction(String::from("JP")), Token::Register(String::from("#end"))]
        ]);
    }
}