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

    // code generator
    let mut codegen = CodeGenerator::new();

    // iterate over the expressions
    for expr in exprs.iter() {
        // check if they are valid
        semantics::check(expr).unwrap();
        // and pass the code generator
        codegen.add(expr);
    }

    Ok(vec![])
}
