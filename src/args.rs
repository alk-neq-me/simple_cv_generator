use clap::Parser;

use crate::models::base::FreeTemplate;


#[derive(Parser)]
#[command(
    version="1.0",
    author="Aung Koko Lwin",
    about="A terminal-based resume generator"
)]
pub struct Args {
    #[arg(short, long)]
    pub config_file: String,

    #[arg(short, long)]
    pub output_file: Option<String>,

    #[arg(short, long)]
    #[clap(value_enum)]
    pub template: FreeTemplate,

}


pub fn parse_args() -> Args {
    Args::parse()
}
