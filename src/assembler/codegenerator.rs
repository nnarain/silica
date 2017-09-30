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
    incomplete_queue: Vec<IncompleteInstruction>,
    largest_address: u32
}


impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            address_counter: 0,
            labels: HashMap::new(),
            opcodes: vec![0; 4096],
            incomplete_queue: vec![],
            largest_address: 0
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

        let reduced_mem = self.reduce_memory_size();

        if reduced_mem.len() < 0x200 {
            return reduced_mem
        }
        else {
            let mut output = vec![0; reduced_mem.len() - 0x200];

            for i in 0..output.len() {
                output[i] = reduced_mem[i + 0x200];
            }
            return output
        }
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
                "db" => {
                    for i in 1..expr.len() {
                        if let Token::NumericLiteral(n) = expr[i] {
                            self.opcodes[self.address_counter as usize] = n as u8;
                            self.increment_address_counter(1);
                        }
                    }
                }
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
                "RND" => self.process_rnd_instruction(expr),
                "DRW" => self.process_draw_instruction(expr),
                "LD" => self.process_load_instruction(expr),
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
                self.queue_incomplete_instruction(expr);
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

    fn process_rnd_instruction(&mut self, expr: &Expression) {
        if let Token::Register(ref reg) = expr[1] {
            if let Token::NumericLiteral(n) = expr[2] {
                let reg_num = self.register_name_to_u8(reg);
                self.append_opcode(0xC0 | reg_num, n as u8);
            }
        }
    }

    fn process_draw_instruction(&mut self, expr: &Expression) {
        if let Token::Register(ref reg1) = expr[1] {
            if let Token::Register(ref reg2) = expr[2] {
                if let Token::NumericLiteral(n) = expr[3] {
                    let reg1_num = self.register_name_to_u8(reg1);
                    let reg2_num = self.register_name_to_u8(reg2);

                    self.append_opcode(0xD0 | reg1_num, (reg2_num << 4) | n as u8);
                }
            }
        }
    }

    fn process_load_instruction(&mut self, expr: &Expression) {
        if let Token::Register(ref reg1) = expr[1] {
            if expr[1].is_general_purpose_register() {
                // operand 1 is a general purpose register
                let reg1_num = self.register_name_to_u8(reg1);

                if let Token::NumericLiteral(kk) = expr[2] {
                   // println!("{:X} {:X}", 0x60 | reg1_num, kk as u8);
                    self.append_opcode(0x60 | reg1_num, kk as u8);
                }
                else if let Token::Register(ref reg2) = expr[2] {
                    if expr[2].is_general_purpose_register() {
                        let reg2_num = self.register_name_to_u8(reg2);
                        self.append_opcode(0x80 | reg1_num, reg2_num << 4);
                    }
                    else {
                        match reg2.as_ref() {
                            "DT" => self.append_opcode(0xF0 | reg1_num, 0x07),
                            "K" => self.append_opcode(0xF0 | reg1_num, 0x0A),
                            "[I]" => self.append_opcode(0xF0 | reg1_num, 0x65),
                            _ => {
                                panic!("Invalid operand");
                            }
                        }
                    }
                }
            }
            else {
                // operand 1 is not a general purpose register

                if let Token::Register(ref reg2) = expr[2] {
                    let reg2_num = self.register_name_to_u8(reg2);

                    match reg1.as_ref() {
                        "DT" => self.append_opcode(0xF0 | reg2_num, 0x15),
                        "ST" => self.append_opcode(0xF0 | reg2_num, 0x18),
                        "F" => self.append_opcode(0xF0 | reg2_num, 0x29),
                        "B" => self.append_opcode(0xF0 | reg2_num, 0x33),
                        "[I]" => self.append_opcode(0xF0 | reg2_num, 0x55),
                        _ => panic!("Invalid operand for instruction LD")
                    }
                }
                else if let Token::NumericLiteral(nnn) = expr[2] {
                    match reg1.as_ref() {
                        "I" => self.append_opcode(0xA0 | (nnn >> 8) as u8, (nnn & 0xFF) as u8),
                        _ => {
                            panic!("Invalid operand for instruction LD");
                        }
                    }
                } else if let Token::LabelOperand(ref label) = expr[2] {
                    match reg1.as_ref() {
                        "I" => {
                            // see if the address has been stored
                            if self.labels.contains_key(label) {
                                let address = self.labels[label];
                                self.append_opcode(0xA0 | (address >> 8) as u8, (address & 0xFF) as u8); 
                            }
                            else {
                                // if the address has not been encountered, queue as incomplete
                                self.queue_incomplete_instruction(expr);
                            }
                        },
                        _ => {
                            panic!("Invalid operand for instruction LD");
                        }
                    }
                }
            }
        }
    }

    fn queue_incomplete_instruction(&mut self, expr: &Expression) {
        let incomplete = IncompleteInstruction::new(self.address_counter, expr.clone());
        self.incomplete_queue.push(incomplete);
        self.increment_address_counter(2);
    }

    fn reduce_memory_size(&mut self) -> Vec<u8> {
        self.opcodes.drain(..self.largest_address as usize).collect()
    }

    fn append_opcode(&mut self, msb: u8, lsb: u8) {
        println!("[{:X}] = {:X} {:X}", self.address_counter, msb, lsb);
        self.opcodes[self.address_counter as usize] = msb;
        self.opcodes[(self.address_counter + 1) as usize] = lsb;
        self.increment_address_counter(2);
    }

    fn increment_address_counter(&mut self, i: u32) {
        self.address_counter += i;
        if self.address_counter > self.largest_address {
            self.largest_address = self.address_counter;
        }
    }

    fn register_name_to_u8(&mut self, name: &String) -> u8 {
        name[1..].to_string().parse::<u8>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_directive() {
        let expr = vec![
            Token::Directive(String::from("db")),
            Token::NumericLiteral(0x00),
            Token::NumericLiteral(0x01),
            Token::NumericLiteral(0x02),
            Token::NumericLiteral(0x03)                                    
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0x00);
        assert_eq!(opcodes[1], 0x01);
        assert_eq!(opcodes[2], 0x02);
        assert_eq!(opcodes[3], 0x03);        
    }

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

    #[test]
    fn test_rnd() {
        let expr = vec![
            Token::Instruction(String::from("RND")),
            Token::Register(String::from("V0")),
            Token::NumericLiteral(0xFF)
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0xC0);
        assert_eq!(opcodes[1], 0xFF);
    }

    #[test]
    fn test_drw() {
        let expr = vec![
            Token::Instruction(String::from("DRW")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("V1")),
            Token::NumericLiteral(0xF)
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0xD0);
        assert_eq!(opcodes[1], 0x1F);
    }

    #[test]
    fn test_ld1() {
        let expr = vec![
            Token::Instruction(String::from("LD")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("V1"))
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0x80);
        assert_eq!(opcodes[1], 0x10);
    }

    #[test]
    fn test_ld2() {
        let expr = vec![
            Token::Instruction(String::from("LD")),
            Token::Register(String::from("V0")),
            Token::NumericLiteral(0xFF)
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0x60);
        assert_eq!(opcodes[1], 0xFF);
    }

    #[test]
    fn test_ld3() {
        let expr = vec![
            Token::Instruction(String::from("LD")),
            Token::Register(String::from("I")),
            Token::NumericLiteral(0xFFF)
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0xAF);
        assert_eq!(opcodes[1], 0xFF);
    }

    #[test]
    fn test_ld4() {
        let expr = vec![
            Token::Instruction(String::from("LD")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("DT"))
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0xF0);
        assert_eq!(opcodes[1], 0x07);
    }

    #[test]
    fn test_ld5() {
        let expr = vec![
            Token::Instruction(String::from("LD")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("K"))
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0xF0);
        assert_eq!(opcodes[1], 0x0A);
    }

    #[test]
    fn test_ld6() {
        let expr = vec![
            Token::Instruction(String::from("LD")),
            Token::Register(String::from("DT")),
            Token::Register(String::from("V0"))
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0xF0);
        assert_eq!(opcodes[1], 0x15);
    }

    #[test]
    fn test_ld7() {
        let expr = vec![
            Token::Instruction(String::from("LD")),
            Token::Register(String::from("ST")),
            Token::Register(String::from("V0"))
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0xF0);
        assert_eq!(opcodes[1], 0x18);
    }

    #[test]
    fn test_ld8() {
        let expr = vec![
            Token::Instruction(String::from("LD")),
            Token::Register(String::from("F")),
            Token::Register(String::from("V0"))
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0xF0);
        assert_eq!(opcodes[1], 0x29);
    }

    #[test]
    fn test_ld9() {
        let expr = vec![
            Token::Instruction(String::from("LD")),
            Token::Register(String::from("B")),
            Token::Register(String::from("V0"))
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0xF0);
        assert_eq!(opcodes[1], 0x33);
    }

    #[test]
    fn test_ld10() {
        let expr = vec![
            Token::Instruction(String::from("LD")),
            Token::Register(String::from("[I]")),
            Token::Register(String::from("V0"))
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0xF0);
        assert_eq!(opcodes[1], 0x55);
    }

    #[test]
    fn test_ld11() {
        let expr = vec![
            Token::Instruction(String::from("LD")),
            Token::Register(String::from("V0")),
            Token::Register(String::from("[I]"))
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![expr]);

        assert_eq!(opcodes[0], 0xF0);
        assert_eq!(opcodes[1], 0x65);
    }

    #[test]
    fn test_ld_i_addr() {
        let expr = vec![
            Token::Instruction(String::from("LD")),
            Token::Register(String::from("I")), 
            Token::LabelOperand(String::from("label"))
        ];

        let codegen = CodeGenerator::new();
        let opcodes = codegen.generate(vec![
            vec![Token::Label(String::from("label"))],
            expr
        ]);

        assert_eq!(opcodes[0], 0xA0);
        assert_eq!(opcodes[1], 0x00);
    }
}
