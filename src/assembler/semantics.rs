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
                return Err(SemanticsError{message: String::from("Invalid number of tokens for directive expression")})
            }
            match expr[1] {
                Token::NumericLiteral(_) => {
                    return Ok(())
                },
                _ => return Err(SemanticsError{message: String::from("Invalid token in directive expression")})
            }
        },
        Token::Instruction(ref instr) => {
            check_instruction_semantics(instr, expr)
        },
        _ => {
            Err(SemanticsError{message: String::from("Invalid start of expression")})
        }
    }
}

fn check_instruction_semantics(instr: &String, expr: &Expression) -> Result<(), SemanticsError> {
    match instr.as_ref() {
        "CLS" => {
            if expr.len() == 1 {
                Ok(())
            }
            else {
                Err(SemanticsError{message: String::from("CLS has no operands")})
            }
        },
        "RET" => {
            if expr.len() == 1 {
                Ok(())
            }
            else {
                Err(SemanticsError{message: String::from("RET has no operands")})
            }
        },
        "JP" => {
            if expr.len() == 2 {
                if expr[1].is_numeric_literal() || expr[1].is_label_operand() {
                    Ok(())
                }
                else {
                    Err(SemanticsError{message: String::from("Invalid operand for instruction JP")})
                }
            }
            else {
                Err(SemanticsError{message: String::from("Invalid number of operands for JP expression")})
            }
        },
        "JR" => {
            if expr.len() == 2 {
                if expr[1].is_numeric_literal() || expr[1].is_label_operand() {
                    Ok(())
                }
                else {
                    Err(SemanticsError{message: String::from("Invalid operand for instruction JR")})
                }
            }
            else {
                Err(SemanticsError{message: String::from("Invalid number of operands for JR expression")})
            }
        },
        "CALL" => {
            if expr.len() == 2 {
                if expr[1].is_numeric_literal() || expr[1].is_label_operand() {
                    Ok(())
                }
                else {
                    Err(SemanticsError{message: String::from("Invalid operand for instruction CALL")})
                }
            }
            else {
                Err(SemanticsError{message: String::from("Invalid number of operands for CALL expression")})
            }
        },
        "SE" => {
            if expr.len() == 3 {
                if (expr[1].is_register() && expr[2].is_numeric_literal()) ||
                    (expr[1].is_register() && expr[2].is_register()) {
                    Ok(())
                }
                else {
                    Err(SemanticsError{message: String::from("Invalid operands for SE instruction")})
                }
            }
            else {
                Err(SemanticsError{message: String::from("Invalid number of operands for SE expression")})                
            }
        },
        "SNE" => {
            if expr.len() == 3 {
                if (expr[1].is_register() && expr[2].is_numeric_literal()) ||
                    (expr[1].is_register() && expr[2].is_register()) {
                    Ok(())
                }
                else {
                    Err(SemanticsError{message: String::from("Invalid operands for SNE instruction")})
                }
            }
            else {
                Err(SemanticsError{message: String::from("Invalid number of operands for SNE expression")})                
            }
        },
        "LD" => {
            if expr.len() == 3 {
                if (expr[1].is_register() && expr[2].is_numeric_literal()) ||
                    (expr[1].is_register() && expr[2].is_register()) {
                    Ok(())
                }
                else {
                    Err(SemanticsError{message: String::from("Invalid operands for LD instruction")})
                }
            }
            else {
                Err(SemanticsError{message: String::from("Invalid number of operands for LD expression")})                
            }
        },
        "OR" => {
            if expr.len() == 3 {
                if expr[1].is_register() && expr[2].is_register() {
                    Ok(())
                }
                else {
                    Err(SemanticsError{message: String::from("Invalid operands for OR instruction")})
                }
            }
            else {
                Err(SemanticsError{message: String::from("Invalid number of operands for OR expression")})                
            }
        },
        "AND" => {
            if expr.len() == 3 {
                if expr[1].is_register() && expr[2].is_register() {
                    Ok(())
                }
                else {
                    Err(SemanticsError{message: String::from("Invalid operands for AND instruction")})
                }
            }
            else {
                Err(SemanticsError{message: String::from("Invalid number of operands for AND expression")})                
            }
        },
        "XOR" => {
            if expr.len() == 3 {
                if expr[1].is_register() && expr[2].is_register() {
                    Ok(())
                }
                else {
                    Err(SemanticsError{message: String::from("Invalid operands for XOR instruction")})
                }
            }
            else {
                Err(SemanticsError{message: String::from("Invalid number of operands for XOR expression")})                
            }
        },
        "ADD" => {
            if expr.len() == 3 {
                if expr[1].is_register() && expr[2].is_register() ||
                   expr[1].is_register() && expr[2].is_numeric_literal() {
                    Ok(())
                }
                else {
                    Err(SemanticsError{message: String::from("Invalid operands for ADD instruction")})
                }
            }
            else {
                Err(SemanticsError{message: String::from("Invalid number of operands for ADD expression")})                
            }
        },
        "SUB" => {
            if expr.len() == 3 {
                if expr[1].is_register() && expr[2].is_register() {
                    Ok(())
                }
                else {
                    Err(SemanticsError{message: String::from("Invalid operands for SUB instruction")})
                }
            }
            else {
                Err(SemanticsError{message: String::from("Invalid number of operands for SUB expression")})                
            }
        },
        "SUBN" => {
            if expr.len() == 3 {
                if expr[1].is_register() && expr[2].is_register() {
                    Ok(())
                }
                else {
                    Err(SemanticsError{message: String::from("Invalid operands for SUBN instruction")})
                }
            }
            else {
                Err(SemanticsError{message: String::from("Invalid number of operands for SUBN expression")})                
            }
        },
        "SHL" => {
            if expr.len() == 3 {
                if expr[1].is_register() && expr[2].is_register() {
                    Ok(())
                }
                else {
                    Err(SemanticsError{message: String::from("Invalid operands for SHL instruction")})
                }
            }
            else {
                Err(SemanticsError{message: String::from("Invalid number of operands for SHL expression")})                
            }
        },
        "SHR" => {
            if expr.len() == 3 {
                if expr[1].is_register() && expr[2].is_register() {
                    Ok(())
                }
                else {
                    Err(SemanticsError{message: String::from("Invalid operands for SHR instruction")})
                }
            }
            else {
                Err(SemanticsError{message: String::from("Invalid number of operands for SHR expression")})                
            }
        },
        "RND" => {
            if expr.len() == 3 {
                if expr[1].is_register() && expr[2].is_numeric_literal() {
                    Ok(())
                }
                else {
                    Err(SemanticsError{message: String::from("Invalid operands for RND instruction")})
                }
            }
            else {
                Err(SemanticsError{message: String::from("Invalid number of operands for RND expression")})                
            }
        },
        "DRW" => {
            if expr.len() == 4 {
                if expr[1].is_register() && expr[2].is_register() && expr[3].is_numeric_literal() {
                    Ok(())
                }
                else {
                    Err(SemanticsError{message: String::from("Invalid operands for DRW instruction")})
                }
            }
            else {
                Err(SemanticsError{message: String::from("Invalid number of operands for DRW expression")})                
            }
        },
        "SKP" => {
            if expr.len() == 2 {
                if expr[1].is_register() {
                    Ok(())
                }
                else {
                    Err(SemanticsError{message: String::from("Invalid operands for SKP instruction")})
                }
            }
            else {
                Err(SemanticsError{message: String::from("Invalid number of operands for SKP expression")})                
            }
        },
        "SKNP" => {
            if expr.len() == 2 {
                if expr[1].is_register() {
                    Ok(())
                }
                else {
                    Err(SemanticsError{message: String::from("Invalid operands for SKNP instruction")})
                }
            }
            else {
                Err(SemanticsError{message: String::from("Invalid number of operands for SKNP expression")})                
            }
        },
        _ => Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_label() {
        let expr = vec![Token::Label(String::from("CLS"))];
        check(&expr).unwrap();
    }

    #[test]
    fn test_check_directive() {
        let expr = vec![
            Token::Directive(String::from("org")),
            Token::NumericLiteral(0x200)
        ];
        check(&expr).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_check_directive_only() {
        let expr = vec![
            Token::Directive(String::from("org"))
        ];
        check(&expr).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_check_directive_invalid_operand() {
        let expr = vec![
            Token::Directive(String::from("org")),
            Token::Register(String::from("V0"))
        ];
        check(&expr).unwrap();
    }

    #[test]
    fn test_check_instruction1() {
        let expr = vec![
            Token::Instruction(String::from("CLS"))
        ];
        check(&expr).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_check_instruction2() {
        let expr = vec![
            Token::Instruction(String::from("CLS")),
            Token::NumericLiteral(0)
        ];
        check(&expr).unwrap();
    }

    #[test]
    fn test_check_jp1() {
        let expr = vec![
            Token::Instruction(String::from("JP")),
            Token::NumericLiteral(0x200)
        ];
        check(&expr).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_check_jp2() {
        let expr = vec![
            Token::Instruction(String::from("JP")),
            Token::Label(String::from("start"))
        ];
        check(&expr).unwrap();
    }

    #[test]
    fn test_check_jr() {
        let expr = vec![
            Token::Instruction(String::from("JR")),
            Token::LabelOperand(String::from("start"))
        ];
        check(&expr).unwrap();
    }
    
    #[test]
    fn test_check_se1() {
        let expr = vec![
            Token::Instruction(String::from("SE")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("V1")),            
        ];
        check(&expr).unwrap();
    }
    
    #[test]
    fn test_check_se2() {
        let expr = vec![
            Token::Instruction(String::from("SE")),
            Token::Register(String::from("V0")),
            Token::NumericLiteral(0xFF)
        ];
        check(&expr).unwrap();
    }

    #[test]
    fn test_check_sne1() {
        let expr = vec![
            Token::Instruction(String::from("SNE")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("V1")),            
        ];
        check(&expr).unwrap();
    }

    #[test]
    fn test_check_sne2() {
        let expr = vec![
            Token::Instruction(String::from("SNE")),
            Token::Register(String::from("V0")),
            Token::NumericLiteral(0xFF)
        ];
        check(&expr).unwrap();
    }


    #[test]
    fn test_check_ld1() {
        let expr = vec![
            Token::Instruction(String::from("SNE")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("V1")),            
        ];
        check(&expr).unwrap();
    }

    #[test]
    fn test_check_ld2() {
        let expr = vec![
            Token::Instruction(String::from("SNE")),
            Token::Register(String::from("V0")),
            Token::NumericLiteral(0xFF)
        ];
        check(&expr).unwrap();
    }

    #[test]
    fn test_check_or() {
        let expr = vec![
            Token::Instruction(String::from("OR")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("V1")),            
        ];
        check(&expr).unwrap();
    }

    #[test]
    fn test_check_and() {
        let expr = vec![
            Token::Instruction(String::from("AND")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("V1")),            
        ];
        check(&expr).unwrap();
    }

    #[test]
    fn test_check_xor() {
        let expr = vec![
            Token::Instruction(String::from("XOR")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("V1")),            
        ];
        check(&expr).unwrap();
    }

    #[test]
    fn test_check_add1() {
        let expr = vec![
            Token::Instruction(String::from("ADD")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("V1")),            
        ];
        check(&expr).unwrap();
    }

    #[test]
    fn test_check_add2() {
        let expr = vec![
            Token::Instruction(String::from("ADD")),
            Token::Register(String::from("V0")),
            Token::NumericLiteral(1)            
        ];
        check(&expr).unwrap();
    }

    #[test]
    fn test_check_sub() {
        let expr = vec![
            Token::Instruction(String::from("SUB")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("V1")),            
        ];
        check(&expr).unwrap();
    }

    #[test]
    fn test_check_subn() {
        let expr = vec![
            Token::Instruction(String::from("SUBN")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("V1")),            
        ];
        check(&expr).unwrap();
    }

    #[test]
    fn test_check_shl() {
        let expr = vec![
            Token::Instruction(String::from("SHL")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("V1")),            
        ];
        check(&expr).unwrap();
    }

    #[test]
    fn test_check_shr() {
        let expr = vec![
            Token::Instruction(String::from("SHR")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("V1")),            
        ];
        check(&expr).unwrap();
    }

    #[test]
    fn test_check_rnd() {
        let expr = vec![
            Token::Instruction(String::from("RND")),
            Token::Register(String::from("V0")),
            Token::NumericLiteral(1)            
        ];
        check(&expr).unwrap();
    }

    #[test]
    fn test_check_drw() {
        let expr = vec![
            Token::Instruction(String::from("DRW")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("V1")),            
            Token::NumericLiteral(5) 
        ];
        check(&expr).unwrap();
    }

    #[test]
    fn test_check_skp() {
        let expr = vec![
            Token::Instruction(String::from("SKP")),
            Token::Register(String::from("V0"))
        ];
        check(&expr).unwrap();
    }

    #[test]
    fn test_check_sknp() {
        let expr = vec![
            Token::Instruction(String::from("SKNP")),
            Token::Register(String::from("V0"))
        ];
        check(&expr).unwrap();
    }
}