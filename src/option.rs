use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Opt {
    #[arg(short, long)]
    pub output: String,

    #[arg(short, long)]
    pub url: String,
}