use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version="1.0",
    author="Aung Koko Lwin",
    about="A terminal-based resume generator"
)]
pub struct Args {
    #[arg(short, long)]
    pub config_file: String,

    #[arg(short, long)]
    pub output_file: Option<String>
}


pub fn parse_args() -> Args {
    Args::parse()
}
