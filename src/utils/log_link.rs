use std::{path::Path, fs};
use serde::{Serialize, de::DeserializeOwned};

use super::{template::generate_context, save::save};

pub fn log_link<T: Serialize + DeserializeOwned>(file_base: &str, config_file: &str, template_file: &str) {
    let cv = generate_context::<T>(config_file, template_file).unwrap();

    let dir = Path::new(file_base).parent().unwrap();
    if !Path::exists(dir) {
        fs::create_dir(dir).expect("Failed creating output dir");
    }

    match save(cv, file_base) {
        Ok(path) => println!("file://{}", path.display()),
        Err(err) => println!("{err:?}")
    };
}


