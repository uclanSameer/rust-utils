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

/**
 * This program will create thumbnail of the given image
 */
//
// pub mod create_thumbnail;
// fn main() {
//     create_thumbnail::create_thumbnail();
// }

/**
 * This program will create http server using axum
 */
pub mod server;
pub mod rayon_tests;
use server::start_server;


#[tokio::main]
async fn main() {
    // build our application with a route
    start_server().await;
}
