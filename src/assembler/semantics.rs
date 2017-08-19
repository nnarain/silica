use assembler::lexer::Token;
use assembler::parser::Expression;

#[derive(Debug)]
pub struct SemanticsError {
    message: String
}

/// Check an expression and ensure it is semantically correct
pub fn check(expr: &Expression) -> Result<(), SemanticsError> {
    match expr[0] {
        // all labels are correct
        Token::Label(_) => {
            Ok(())
        },
        // check a directive can only have a numeric literal operand
        Token::Directive(_) => {
            if expr.len() != 2 {
                return Err(SemanticsError{message: String::from("Too many tokens for directive expression")})
            }
            match expr[1] {
                Token::NumericLiteral(_) => {
                    return Ok(())
                },
                _ => return Err(SemanticsError{message: String::from("Invalid token in directive expression")})
            }
        },
        Token::Instruction(_) => {
            Ok(())
        },
        _ => {
            Err(SemanticsError{message: String::from("Invalid start of expression")})
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    
}