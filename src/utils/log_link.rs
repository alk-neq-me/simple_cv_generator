use serde::{Serialize, de::DeserializeOwned};

use super::{template::generate_context, save::save};

pub fn log_link<T: Serialize + DeserializeOwned>(file_base: &str, config_file: &str, template_file: &str) {
    let cv = generate_context::<T>(config_file, template_file).unwrap();
    match save(cv, file_base) {
        Ok(path) => println!("file://{}", path.display()),
        Err(err) => println!("{err:?}")
    };
}


