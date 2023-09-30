use serde::{Serialize, Deserialize};
use tera::{Context, Error};
use clap::ValueEnum;


// #[derive(Serialize, Deserialize, Debug)]
// pub struct Base<T> {
//     pub config: HashMap<String, T>
// }

#[derive(ValueEnum, Clone)]
pub enum FreeTemplate {
    Basic,
    Xross,
    CareerCraft,
}

#[derive(Deserialize, Serialize)]
pub struct Education {
    title: String,
    description: String,
}

#[derive(Deserialize, Serialize)]
pub struct Language {
    title: String,
}

#[derive(Deserialize, Serialize)]
pub struct ComputerSkill {
    title: String,
}

#[derive(Deserialize, Serialize)]
pub struct Experence {
    title: String,
    description: String,
}

#[derive(Deserialize, Serialize)]
pub struct OtherSkill {
    title: String
}

#[derive(Deserialize, Serialize)]
pub struct Qualifacation {
    title: String
}

pub fn get_context(template: impl Serialize) -> Result<Context, Error> {
    Context::from_serialize(template)
}
