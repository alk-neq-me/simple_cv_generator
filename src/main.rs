mod utils;
mod models;
mod args;

use args::parse_args;
use models::{base::FreeTemplate, basic::Basic, xross::Xross, career_craft::CareerCraft};
use utils::log_link::log_link;


fn main() {
    let args = parse_args();
    let file_base = args.output_file.unwrap_or(format!("{}.html", args.config_file.split('.').next().unwrap().to_owned()));

    let config_file = args.config_file;

    match args.template {
        FreeTemplate::Basic => log_link::<Basic>(&file_base, &config_file, "basic/basic.html"),
        FreeTemplate::Xross => log_link::<Xross>(&file_base, &config_file, "xross/xross.html"),
        FreeTemplate::CareerCraft => log_link::<CareerCraft>(&file_base, &config_file, "careerCraft/career-craft.html")
    }
}
