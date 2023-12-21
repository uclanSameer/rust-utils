use clap::Parser;


pub mod convert_to_struct;
pub mod console;

fn main() {
    let args = console::Args::parse();
    convert_to_struct::json_to_struct("test.json", "Map")
}
