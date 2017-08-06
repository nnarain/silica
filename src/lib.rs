#[macro_use]
extern crate serde_derive;
extern crate docopt;

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
