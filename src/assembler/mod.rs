mod lexer;

use std::io::Error;

/// consume input data and assemble the code
pub fn assemble(input_data: Vec<u8>) -> Result<Vec<u8>, Error> {
    let tokens = lexer::tokenize(&input_data[..]).unwrap();
    println!("{:?}", tokens);

    Ok(vec![])
}
