use assembler::lexer::Token;
use assembler::parser::Expression;

use std::collections::HashMap;

/// Contains the logic to transform valid expressions of Tokens into
/// Chip8 opcodes
pub struct CodeGenerator {
    address_counter: u32,
    labels: HashMap<String, u32>
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            address_counter: 0,
            labels: HashMap::new()
        }
    }

    /// Process a new expression
    pub fn add(&mut self, expr: &Expression) {
        match expr[0] {
            Token::Directive(_) => {
                self.process_directive(expr);
            },
            Token::Label(_) => {
                self.process_label(expr);
            },
            Token::Instruction(_) => {
                self.process_instruction(expr);
            },
            _ => {
                panic!("Invalid token for start of expression");
            }
        }
    }

    fn process_directive(&mut self, expr: &Expression) {
        if let Token::Directive(ref directive) = expr[0] {
            match directive.as_ref() {
                "org" => {
                    if let Token::NumericLiteral(address) = expr[1] {
                        // set the new address location
                        self.address_counter = address;
                    }
                },
                _ => {}
            }
        }
    }

    fn process_label(&mut self, expr: &Expression) {
        if let Token::Label(ref label) = expr[0] {
            if !self.labels.contains_key(label) {
                self.labels.insert((*label).clone(), self.address_counter);
            }
            else {
                panic!("The label: {} has already been used", label);
            }
        }
    }

    fn process_instruction(&mut self, expr: &Expression) {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_address() {
        let expr = vec![
            Token::Directive(String::from("org")),
            Token::NumericLiteral(0x200)
        ];

        let mut codegen = CodeGenerator::new();
        codegen.add(&expr);

        assert_eq!(codegen.address_counter, 0x200);
    }
}