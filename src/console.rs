use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub file: String,

    #[clap(short, long)]
    pub name: String,
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ThumbNailArgs {
    #[clap(short, long)]
    pub image: String
}