mod utils;
mod models;

use utils::{template::generate_context, save::save};

use crate::models::basic::Basic;

fn main() {
    let cv = generate_context::<Basic>("me.toml").unwrap();
    match save(cv, "me.html") {
        Ok(path) => println!("file://{}", path.display()),
        Err(err) => println!("{err:?}")
    };
}
