use assembler::lexer::*;
use std::io::Error;

use nom::*;

/// an expression is a certain combination of tokens
type Expression = Vec<Token>;

fn take_token(tokens: &[Token]) -> IResult<&[Token], &[Token]> {
    if tokens.len() > 0 {
        match token[0] {
            Token::Label(_) => {
                IResult::Done(&tokens[1..], &tokens[0..1])
            },
            _ => {
                IResult::Error()
            }
        }
    }
    else {
        IResult::Incomplete(Needed::Size(1))
    }
}

/// parse labels from tokens
named!(parse_label<&[Token], Expression>,
    
);

/// parse expressions from tokens
pub fn parse_expressions(tokens: Vec<Token>) -> Result<Vec<Expression>, Error> {
    Ok(vec![])
}

#[cfg(test)]
mod tests {

}