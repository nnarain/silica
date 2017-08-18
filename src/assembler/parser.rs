use assembler::lexer::*;
use std::io::Error;

use nom::*;

/// an expression is a certain combination of tokens
pub type Expression = Vec<Token>;

macro_rules! tag_token {
    ($i: expr, $tag: pat) => (
        {
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
            }
        }
    )
}

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

/// parse expressions from tokens
pub fn parse_expressions(tokens: Vec<Token>) -> Result<Vec<Expression>, Error> {
    Ok(vec![])
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
}