mod lexer;

/// consume input data and assemble the code
pub fn assemble(input_data: Vec<u8>) {
    let tokens = lexer::tokenize(&input_data[..]).unwrap();
    println!("{:?}", tokens);
}
