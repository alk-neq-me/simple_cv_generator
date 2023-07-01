mod utils;
mod models;
mod args;

use args::parse_args;
use utils::{template::generate_context, save::save};

use crate::models::basic::Basic;

fn main() {
    let args = parse_args();
    let file_base = args.output_file.unwrap_or(format!("{}.html", args.config_file.split('.').next().unwrap().to_owned()));

    let cv = generate_context::<Basic>(&args.config_file).unwrap();
    match save(cv, &file_base) {
        Ok(path) => println!("file://{}", path.display()),
        Err(err) => println!("{err:?}")
    };
}
