use clap::Parser;

/**
 * This struct is used to parse the command line arguments
 * Eg:
 * $ cargo run -- -i input.json -n Person
 * OR
 * ./executable -i input.json -n Person
 */
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub file: String,

    #[clap(short, long)]
    pub name: String,
}

/**
 * This struct is used to parse the command line arguments
 * Eg:
 * $ cargo run -- -i input.jpg -o output.jpg
 * OR
 * ./executable -i input.jpg -o output.jpg // output.jpg is optional
 * If output is not given then it will create thumbnail.png
 */
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ThumbNailArgs {
    #[clap(short, long)]
    pub image: String,

    #[clap(short, long)]
    pub output: Option<String>,
}
