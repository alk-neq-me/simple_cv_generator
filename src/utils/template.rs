use lazy_static::lazy_static;
use serde::de::DeserializeOwned;
use tera::{Context, Tera, Error};
use toml;
use std::fs;
use std::process::exit;


lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let Ok(tera) = Tera::new("templates/**/*.html") else {
            panic!("There was an error loading the templates");
        };
        tera
    };
}

pub trait Generate {
    fn get_context(&self) -> Result<Context, Error>;
}

pub fn generate_context<T: Generate + DeserializeOwned>(config_file: &str) -> Result<String, Error> {
    let config = match fs::read_to_string(config_file) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("could not read file {err:?}");
            exit(1);
        }
    };

    let me: Box<dyn Generate> = match toml::from_str::<T>(&config) {
        Ok(data) => Box::new(data),
        Err(err) => {
            eprintln!("could not load toml {err:?}");
            exit(1);
        }
    };

    let context = me.get_context()?;
    Ok(TEMPLATES.render("basic.html", &context).unwrap())
}
