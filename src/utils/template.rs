use lazy_static::lazy_static;
use serde::Serialize;
use serde::de::DeserializeOwned;
use tera::{Tera, Error};
use toml;
use std::fs;
use std::process::exit;

use crate::models::base::get_context;


lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let Ok(tera) = Tera::new("templates/**/*.html") else {
            panic!("There was an error loading the templates");
        };
        tera
    };
}


pub fn generate_context<T: DeserializeOwned + Serialize>(config_file: &str, template_file: &str) -> Result<String, Error> {
    let config = match fs::read_to_string(config_file) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("could not read file {err:?}");
            exit(1);
        }
    };

    let me: T = match toml::from_str(&config) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("could not load toml {err:?}");
            exit(1);
        }
    };

    let context = get_context(me)?;
    Ok(TEMPLATES.render(template_file, &context)?)
}
