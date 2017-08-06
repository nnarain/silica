extern crate silica;

fn main() {
    let options = silica::options::get_program_options();
    println!("{:?}", options);
}
