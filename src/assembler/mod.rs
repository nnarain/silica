mod lexer;
mod parser;
mod semantics;
mod codegenerator;

use self::codegenerator::CodeGenerator;

use std::io::Error;

/// consume input data and assemble the code
pub fn assemble(input_data: Vec<u8>) -> Result<Vec<u8>, Error> {
    // TODO: better error handling
    // transform input data into tokens
    let tokens = lexer::tokenize(&input_data[..]).unwrap();
    // transform tokens into expressions
    let exprs = parser::parse(tokens).unwrap();

    // generate opcodes from the expressions
    let codegen = CodeGenerator::new();
    let opcodes = codegen.generate(exprs);

    Ok(opcodes)
}
