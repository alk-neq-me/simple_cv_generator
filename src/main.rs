mod utils;
mod models;
mod args;

use args::parse_args;
use models::{base::FreeTemplate, basic::Basic, xross::Xross, career_craft::CareerCraft};
use utils::log_link::log_link;

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let base: PathBuf = Path::new(".").parent().unwrap().to_path_buf();

    let args = parse_args();
    let file_base = args
        .output_file
        .unwrap_or_else(|| {
            let output_dir: PathBuf = base.join("output");

            if !output_dir.is_dir() {
                if let Err(e) = fs::create_dir(&output_dir) {
                    eprintln!("Failed creating `output`: {}", e);
                }
            }

            let file_name = Path::new(&args.config_file)
                .file_stem()
                .unwrap_or(OsStr::new("output"))
                .to_str()
                .unwrap_or("output");

            format!("{}/{}.html", output_dir.to_string_lossy().to_string(), file_name)
        });

    let config_file = args.config_file;

    match args.template {
        FreeTemplate::Basic => log_link::<Basic>(&file_base, &config_file, "basic/basic.html"),
        FreeTemplate::Xross => log_link::<Xross>(&file_base, &config_file, "xross/xross.html"),
        FreeTemplate::CareerCraft => log_link::<CareerCraft>(&file_base, &config_file, "careerCraft/career-craft.html")
    }
}
