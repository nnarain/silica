use assembler::lexer::Token;
use assembler::parser::Expression;
use assembler::semantics;

use std::collections::HashMap;

/// Incomplete instruction
struct IncompleteInstruction {
    pub address: u32,
    pub expr: Expression
}

impl IncompleteInstruction {
    pub fn new(address: u32, expr: Expression) -> Self {
        IncompleteInstruction {
            address: address,
            expr: expr
        }
    }
}

/// Contains the logic to transform valid expressions of Tokens into
/// Chip8 opcodes
pub struct CodeGenerator {
    address_counter: u32,
    labels: HashMap<String, u32>,
    opcodes: Vec<u8>,
    incomplete_queue: Vec<IncompleteInstruction>
}


impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            address_counter: 0,
            labels: HashMap::new(),
            opcodes: vec![0; 4096],
            incomplete_queue: vec![]
        }
    }

    /// Consumes the code generator and the expressions and return a vetor containing the generated opecodes
    pub fn generate(mut self, exprs: Vec<Expression>) -> Vec<u8> {
        // iterate over the expressions
        for expr in exprs.iter() {
            self.process_expression(expr);
        }

        // perform a second pass of the expressions to add the ones that could not be completed
        while !self.incomplete_queue.is_empty() {
            let item = self.incomplete_queue.remove(0);
            self.address_counter = item.address;
            self.process_expression(&item.expr);
        }

        self.opcodes
    }

    /// Process a new expression
    fn process_expression(&mut self, expr: &Expression) {
        // check that the expression is valid
        semantics::check(expr).unwrap();
        
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
        if let Token::Instruction(ref instr) = expr[0] {
            match instr.as_ref() {
                "CLS" => self.append_opcode(0x00, 0xE0),
                "RET" => self.append_opcode(0x00, 0xEE),
                "JP" => self.process_jump_instruction(0x10u8, expr),
                "JR" => self.process_jump_instruction(0xB0u8, expr),
                "CALL" => self.process_jump_instruction(0x20u8, expr),
                "SE" => self.process_se_instruction(0x30u8, 0x50u8, expr),
                "SNE" => self.process_se_instruction(0x40u8, 0x90u8, expr),
                "OR" => self.process_logical_instruction(0x01, expr),
                "AND" => self.process_logical_instruction(0x02, expr),
                "XOR" => self.process_logical_instruction(0x03, expr),
                "ADD" => self.process_add_instruction(expr),
                "SUB" => self.process_sub_instruction(expr, 0x05),
                "SHR" => self.process_sub_instruction(expr, 0x06),
                "SUBN" => self.process_sub_instruction(expr, 0x07),
                "SHL" => self.process_sub_instruction(expr, 0x0E),
                "SKP" => self.process_skip_instruction(expr, 0x9E),
                "SKNP" => self.process_skip_instruction(expr, 0xA1),
                _ => {} 
            }
        }
    }

    fn process_jump_instruction(&mut self, first: u8, expr: &Expression) {
        // if the operand is a numeric literal the opcode can be generated now
        if let Token::NumericLiteral(nnn) = expr[1] {
            let msb: u8 = first | (((nnn & 0xF00) >> 8) as u8);
            let lsb: u8 = (nnn & 0x0FF) as u8;
            self.append_opcode(msb, lsb);
        }
        // if the operand is a label operand...
        if let Token::LabelOperand(ref label) = expr[1] {
            // see if the address has been stored
            if self.labels.contains_key(label) {
                let address = self.labels[label];
                let msb: u8 = first | (((address & 0xF00) >> 8) as u8);
                let lsb: u8 = (address & 0xFF) as u8;
                self.append_opcode(msb, lsb); 
            }
            else {
                // if the address has not been encountered, queue as incomplete
                let incomplete = IncompleteInstruction::new(self.address_counter, expr.clone());
                self.incomplete_queue.push(incomplete);
            }
        }
    }

    fn process_se_instruction(&mut self, first: u8, second: u8, expr: &Expression) {
        if let Token::Register(ref reg) = expr[1] {
            let reg_num = self.register_name_to_u8(reg);

            if let Token::NumericLiteral(kk) = expr[2] {
                self.append_opcode(first | reg_num, kk as u8);
            }
            else if let Token::Register(ref reg) = expr[3] {
                let operand_reg_num = self.register_name_to_u8(reg);
                self.append_opcode(second | reg_num, operand_reg_num << 4);
            }
        }
    }

    fn process_logical_instruction(&mut self, first: u8, expr: &Expression) {
        if let Token::Register(ref regx) = expr[1] {
            let regx_num = self.register_name_to_u8(regx);

            if let Token::Register(ref regy) = expr[2] {
                let regy_num = self.register_name_to_u8(regy);

                self.append_opcode(0x80 | regx_num, (regy_num << 4) | first);
            }
        }
    }

    fn process_add_instruction(&mut self, expr: &Expression) {
        let ref reg1 = expr[1];
        if reg1.is_general_purpose_register() {
            if let Token::Register(ref reg) = *reg1 {
                let reg1_num = self.register_name_to_u8(reg);
                if let Token::Register(ref reg2) =  expr[2] {
                    let reg2_num = self.register_name_to_u8(reg2);
                    self.append_opcode(0x80u8 | reg1_num, (reg2_num << 4) | 0x04);
                }
                else if let Token::NumericLiteral(kk) = expr[2] {
                    self.append_opcode(0x70u8 | reg1_num, kk as u8);
                }
            }
        }
        else {
            if let Token::Register(ref reg) = expr[2] {
                let reg_num = self.register_name_to_u8(reg);
                self.append_opcode(0xF0u8 | reg_num, 0x1Eu8);
            }
        }
    }

    fn process_sub_instruction(&mut self, expr: &Expression, last: u8) {
        if let Token::Register(ref reg1) = expr[1] {
            if let Token::Register(ref reg2) = expr[2] {
                let reg1_num = self.register_name_to_u8(reg1);
                let reg2_num = self.register_name_to_u8(reg2);

                self.append_opcode(0x80 | reg1_num, last |  (reg2_num << 4));
            }
        }
    }

    fn process_skip_instruction(&mut self, expr: &Expression, last: u8) {
        if let Token::Register(ref reg) = expr[1] {
            let num = self.register_name_to_u8(reg);
            self.append_opcode(0xE0u8 | num, last);
        }
    }

    fn append_opcode(&mut self, msb: u8, lsb: u8) {
        self.opcodes[self.address_counter as usize] = msb;
        self.opcodes[(self.address_counter + 1) as usize] = lsb;
        self.address_counter += 2;
    }

    fn register_name_to_u8(&mut self, name: &String) -> u8 {
        name[1..].to_string().parse::<u8>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jp_instruction() {
        let expr = vec![
            Token::Instruction(String::from("JP")),
            Token::NumericLiteral(0x200)
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0x12);
        assert_eq!(opcodes[1], 0x00);
    }

    #[test]
    fn test_or() {
        let expr = vec![
            Token::Instruction(String::from("OR")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("V1"))            
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0x80);
        assert_eq!(opcodes[1], 0x11);
    }

    #[test]
    fn test_and() {
        let expr = vec![
            Token::Instruction(String::from("AND")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("V1"))            
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0x80);
        assert_eq!(opcodes[1], 0x12);
    }

    #[test]
    fn test_xor() {
        let expr = vec![
            Token::Instruction(String::from("XOR")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("V1"))            
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0x80);
        assert_eq!(opcodes[1], 0x13);
    }

    #[test]
    fn test_add1() {
        let expr = vec![
            Token::Instruction(String::from("ADD")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("V1"))            
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0x80);
        assert_eq!(opcodes[1], 0x14);
    }

    #[test]
    fn test_add2() {
        let expr = vec![
            Token::Instruction(String::from("ADD")),
            Token::Register(String::from("V0")),
            Token::NumericLiteral(0xFF)           
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0x70);
        assert_eq!(opcodes[1], 0xFF);
    }

    #[test]
    fn test_add3() {
        let expr = vec![
            Token::Instruction(String::from("ADD")),
            Token::Register(String::from("I")),
            Token::Register(String::from("V0"))          
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0xF0);
        assert_eq!(opcodes[1], 0x1E);
    }

    #[test]
    fn test_sub() {
        let expr = vec![
            Token::Instruction(String::from("SUB")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("V1"))            
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0x80);
        assert_eq!(opcodes[1], 0x15);
    }

    #[test]
    fn test_shr() {
        let expr = vec![
            Token::Instruction(String::from("SHR")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("V1"))            
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0x80);
        assert_eq!(opcodes[1], 0x16);
    }

    #[test]
    fn test_sh1() {
        let expr = vec![
            Token::Instruction(String::from("SHL")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("V1"))            
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0x80);
        assert_eq!(opcodes[1], 0x1E);
    }

    #[test]
    fn test_subn() {
        let expr = vec![
            Token::Instruction(String::from("SUBN")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("V1"))            
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0x80);
        assert_eq!(opcodes[1], 0x17);
    }

    #[test]
    fn test_skp() {
        let expr = vec![
            Token::Instruction(String::from("SKP")),
            Token::Register(String::from("V0"))         
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0xE0);
        assert_eq!(opcodes[1], 0x9E);
    }

    #[test]
    fn test_sknp() {
        let expr = vec![
            Token::Instruction(String::from("SKNP")),
            Token::Register(String::from("V0"))         
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0xE0);
        assert_eq!(opcodes[1], 0xA1);
    }
}
