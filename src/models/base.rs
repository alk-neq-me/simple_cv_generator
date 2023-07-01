use serde::Serialize;
use tera::{Context, Error};
use clap::ValueEnum;


// #[derive(Serialize, Deserialize, Debug)]
// pub struct Base<T> {
//     pub config: HashMap<String, T>
// }

#[derive(ValueEnum, Clone)]
pub enum FreeTemplate {
    Basic,
    Xross
}

pub fn get_context(template: impl Serialize) -> Result<Context, Error> {
    Context::from_serialize(template)
}
