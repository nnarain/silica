extern crate silica;

use std::process;

fn main() {
    let options = silica::options::get_program_options();
    let input_data = silica::load_file(&options.arg_input).unwrap_or_else(
        |e| {
            println!("Could not load input file: {:?}", e);
            process::exit(1);
        }
    );

    match silica::assembler::assemble(input_data) {
        Ok(data) => {
            println!("{:?}", data)
        },
        Err(e) => println!("{:?}", e)
    }
}
