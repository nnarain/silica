mod lexer;
mod parser;

use std::io::Error;

/// consume input data and assemble the code
pub fn assemble(input_data: Vec<u8>) -> Result<Vec<u8>, Error> {
    // TODO: better errors
    // transform input data into tokens
    let tokens = lexer::tokenize(&input_data[..]).unwrap();
    // transform tokens into expressions
    let exprs = parser::parse_expressions(tokens);

    Ok(vec![])
}
