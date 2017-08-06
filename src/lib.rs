#[macro_use]
extern crate serde_derive;
extern crate docopt;

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

/// Command line arguments
pub mod options {
    use docopt::Docopt;

    const USAGE: &'static str = "
    silica

    Usage:
      silica [--output=<f>] <input>
      silica (-h | --help)

    Options:
      -o --output=<f>  Output file name
      -h --help        Show help.
    ";

    #[derive(Debug, Deserialize)]
    pub struct ProgramOptions {
        pub arg_input: String,
        pub flag_output: Option<String>
    }

    pub fn get_program_options() -> ProgramOptions {
        Docopt::new(USAGE).and_then(|d| d.deserialize()).unwrap_or_else(|e| e.exit())
    }
}

pub fn load_file(rom_file: &String) -> Result<Vec<u8>, Box<Error>> {
    let mut file = File::open(rom_file)?;

    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    Ok(buffer)
}
