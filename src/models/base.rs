use serde::Serialize;
use tera::{Context, Error};

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Base<T> {
//     pub config: T
// }

pub fn get_context(template: impl Serialize) -> Result<Context, Error> {
    Context::from_serialize(template)
}
