

pub mod console;

/**
 * This program is used to convert json to struct
 */

// pub mod convert_to_struct;
// use clap::Parser;

// fn main() {
//     let args = console::Args::parse();
//     let in_file_name = args.file;
//     let struct_name = args.name;
//     convert_to_struct::json_to_struct(&in_file_name, &struct_name);
// }

pub mod create_thumbnail;
fn main() {
    create_thumbnail::create_thumbnail();
}
